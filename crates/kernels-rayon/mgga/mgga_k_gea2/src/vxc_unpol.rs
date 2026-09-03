//! MGGA_K_GEA2 vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_gea2.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn mgga_k_gea2_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = t4 * t4;
            let t6 = f64x8::splat(M_CBRTPI);
            let t8 = t5 * t6 * f64x8::splat(M_PI);
            let t9 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t10 = zeta_threshold - f64x8::splat(1.0);
            let t12 = ((t9).select(t10, (t9).select(-t10, f64x8::splat(0.0))));
            let t13 = f64x8::splat(1.0) + t12;
            let t15 = (simd::cbrt(zeta_threshold));
            let t16 = t15 * t15;
            let t18 = (simd::cbrt(t13));
            let t19 = t18 * t18;
            let t21 = (((t13).simd_le(zeta_threshold)).select(t16 * zeta_threshold, t19 * t13));
            let t22 = (simd::cbrt(v_rho));
            let t23 = t22 * t22;
            let t24 = t21 * t23;
            let t25 = f64x8::splat(M_CBRT6);
            let t26 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t27 = (simd::cbrt(t26));
            let t28 = t27 * t27;
            let t29 = f64x8::splat(1.0) / t28;
            let t30 = t25 * t29;
            let t31 = f64x8::splat(M_CBRT2);
            let t32 = t31 * t31;
            let t33 = v_sigma * t32;
            let t34 = v_rho * v_rho;
            let t36 = f64x8::splat(1.0) / t23 / t34;
            let t40 = v_lapl * t32;
            let t46 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(648.0) * t30 * t33 * t36 + f64x8::splat(5.0) / f64x8::splat(54.0) * t30 * t40 / t23 / v_rho;
            let t50 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t46));
            let tzk0 = f64x8::splat(2.0) * t50;
            acc_zk = tzk0;
            let t52 = t21 / t22;
            let t56 = t34 * v_rho;
            let t58 = f64x8::splat(1.0) / t23 / t56;
            let t65 = -f64x8::splat(5.0) / f64x8::splat(243.0) * t30 * t33 * t58 - f64x8::splat(25.0) / f64x8::splat(162.0) * t30 * t40 * t36;
            let t70 = ((t3).select(f64x8::splat(0.0), t8 * t52 * t46 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t65));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t70 + f64x8::splat(2.0) * t50;
            acc_vrho = tvrho0;
            let t73 = t8 * t21;
            let t76 = t29 * t32;
            let t78 = t73 / t34 * t25 * t76;
            let t80 = ((t3).select(f64x8::splat(0.0), t78 / f64x8::splat(864.0)));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t80;
            acc_vsigma = tvsigma0;
            let t87 = ((t3).select(f64x8::splat(0.0), t73 / v_rho * t25 * t76 / f64x8::splat(72.0)));
            let tvlapl0 = f64x8::splat(2.0) * v_rho * t87;
            acc_vlapl = tvlapl0;
            let tvtau0 = f64x8::splat(0.0);
            acc_vtau = tvtau0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vlapl.into(); vlapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vtau.into(); vtau[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
