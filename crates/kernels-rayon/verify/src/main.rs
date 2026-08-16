//! Bit-exactness gate for the rayon kernel backend.
//!
//! Runs the same functional through the CubeCL kernel and the mechanically
//! translated plain-Rust kernel on identical inputs and requires the outputs to
//! be *bit-identical*, not merely close. Exits non-zero on any difference, so
//! it can gate the migration of the remaining functionals.

use cubecl::prelude::*;

use libxc_kernel_gga_x_pbe::vxc_unpol::gga_x_pbe_vxc_unpol as cube_vxc;
use libxc_rkernel_gga_x_pbe::vxc_unpol::gga_x_pbe_vxc_unpol as rayon_vxc;

const KAPPA: f64 = 0.8040;
const MU: f64 = 0.2195149727645171;
const DT: f64 = 1.0e-32;
const ZT: f64 = 1.0e-15;
const WORKGROUP: u32 = 256;

/// Grid spanning the density range a real DFT calculation sees, plus the
/// awkward edges: threshold crossings, exact cubes, tiny and huge rho.
fn make_grid(n: usize) -> (Vec<f64>, Vec<f64>) {
    let mut s: u64 = 0x243F6A8885A308D3;
    let mut next = || {
        s ^= s >> 12;
        s ^= s << 25;
        s ^= s >> 27;
        (s.wrapping_mul(0x2545F4914F6CDD1D) >> 11) as f64 / (1u64 << 53) as f64
    };
    let mut rho = Vec::with_capacity(n);
    let mut sigma = Vec::with_capacity(n);

    // Deliberate edge cases first.
    let edges: [(f64, f64); 8] = [
        (0.0, 0.0),
        (1e-32, 0.0),
        (2e-32, 1e-40),
        (1.0, 0.0),
        (8.0, 1.0),
        (1e-300, 1e-300),
        (1e300, 1e300),
        (27.0, 1e-8),
    ];
    for (r, sg) in edges {
        rho.push(r);
        sigma.push(sg);
    }
    while rho.len() < n {
        let r: f64 = 10f64.powf(-10.0 + 13.0 * next());
        let s_red = 3.0 * next();
        let kf = (3.0 * std::f64::consts::PI * std::f64::consts::PI * r).powf(1.0 / 3.0);
        let grad = s_red * 2.0 * kf * r;
        rho.push(r);
        sigma.push(grad * grad);
    }
    (rho, sigma)
}

fn run_cube(rho: &[f64], sigma: &[f64]) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let n = rho.len();
    let client = cubecl::cpu::CpuRuntime::client(&cubecl::cpu::CpuDevice);
    let zeros = vec![0f64; n];
    let rh = client.create_from_slice(bytemuck::cast_slice(rho));
    let sh = client.create_from_slice(bytemuck::cast_slice(sigma));
    let zh = client.create_from_slice(bytemuck::cast_slice(&zeros));
    let ah = client.create_from_slice(bytemuck::cast_slice(&zeros));
    let bh = client.create_from_slice(bytemuck::cast_slice(&zeros));
    unsafe {
        cube_vxc::launch_unchecked::<cubecl::cpu::CpuRuntime>(
            &client,
            CubeCount::new_1d((n as u32).div_ceil(WORKGROUP)),
            CubeDim::new_1d(WORKGROUP),
            ArrayArg::from_raw_parts(rh, n),
            ArrayArg::from_raw_parts(sh, n),
            ArrayArg::from_raw_parts(zh.clone(), n),
            ArrayArg::from_raw_parts(ah.clone(), n),
            ArrayArg::from_raw_parts(bh.clone(), n),
            KAPPA, MU, DT, ZT,
        );
    }
    let rd = |h: cubecl::server::Handle| -> Vec<f64> {
        let b = client.read_one(h).expect("readback");
        bytemuck::cast_slice(&b).to_vec()
    };
    (rd(zh), rd(ah), rd(bh))
}

/// Serial sweep over the whole grid.
fn run_rayon_serial(rho: &[f64], sigma: &[f64]) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let n = rho.len();
    let (mut zk, mut vrho, mut vsigma) = (vec![0f64; n], vec![0f64; n], vec![0f64; n]);
    rayon_vxc(rho, sigma, &mut zk, &mut vrho, &mut vsigma, KAPPA, MU, DT, ZT);
    (zk, vrho, vsigma)
}

/// Chunked rayon sweep. Every array here has per-point stride 1 (unpolarized
/// vxc), so all five split at the same boundary; the polarized kernels will
/// need each array split at its own stride, which is the eval layer's job.
fn run_rayon_parallel(rho: &[f64], sigma: &[f64], chunk: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    use rayon::prelude::*;
    let n = rho.len();
    let (mut zk, mut vrho, mut vsigma) = (vec![0f64; n], vec![0f64; n], vec![0f64; n]);
    zk.par_chunks_mut(chunk)
        .zip(vrho.par_chunks_mut(chunk))
        .zip(vsigma.par_chunks_mut(chunk))
        .zip(rho.par_chunks(chunk))
        .zip(sigma.par_chunks(chunk))
        .for_each(|((((z, vr), vs), r), sg)| {
            rayon_vxc(r, sg, z, vr, vs, KAPPA, MU, DT, ZT);
        });
    (zk, vrho, vsigma)
}

fn compare(name: &str, a: &[f64], b: &[f64], field: &str, bad: &mut usize) {
    let mut first: Option<usize> = None;
    let mut count = 0usize;
    for (i, (x, y)) in a.iter().zip(b).enumerate() {
        // Bit comparison, so NaN == NaN and +0 != -0 are both handled exactly.
        if x.to_bits() != y.to_bits() {
            count += 1;
            if first.is_none() {
                first = Some(i);
            }
        }
    }
    if count == 0 {
        println!("  {name:<28} {field:<8} identical ({} values)", a.len());
    } else {
        let i = first.unwrap();
        println!(
            "  {name:<28} {field:<8} MISMATCH in {count} values; first at [{i}]: {:?} vs {:?}",
            a[i], b[i]
        );
        *bad += count;
    }
}

fn main() {
    let n: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(200_000);

    println!("=== rayon backend bit-exactness gate ===");
    println!("functional: gga_x_pbe, vxc, unpolarized, f64");
    println!("grid points: {n} (incl. threshold, zero, subnormal and overflow edges)\n");

    let (rho, sigma) = make_grid(n);
    let (cz, cv, cs) = run_cube(&rho, &sigma);
    let (sz, sv, ss) = run_rayon_serial(&rho, &sigma);
    let (pz, pv, ps) = run_rayon_parallel(&rho, &sigma, 4096);

    let mut bad = 0usize;
    println!("cubecl-cpu vs rayon (serial):");
    compare("cubecl vs rayon-serial", &cz, &sz, "zk", &mut bad);
    compare("cubecl vs rayon-serial", &cv, &sv, "vrho", &mut bad);
    compare("cubecl vs rayon-serial", &cs, &ss, "vsigma", &mut bad);

    println!("\nrayon serial vs rayon chunked (chunk = 4096):");
    compare("serial vs parallel", &sz, &pz, "zk", &mut bad);
    compare("serial vs parallel", &sv, &pv, "vrho", &mut bad);
    compare("serial vs parallel", &ss, &ps, "vsigma", &mut bad);

    println!();
    if bad == 0 {
        println!("PASS: rayon backend is bit-identical to the CubeCL kernel.");
    } else {
        println!("FAIL: {bad} differing values.");
        std::process::exit(1);
    }
}
