//! MGGA_X_2D_JS17 vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_2d_js17.c`
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

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_2d_js17_vxc_unpol(
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
            let t4 = ((f64x8::splat(M_PI)).sqrt());
            let t6 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t7 = zeta_threshold - f64x8::splat(1.0);
            let t9 = ((t6).select(t7, (t6).select(-t7, f64x8::splat(0.0))));
            let t10 = f64x8::splat(1.0) + t9;
            let t12 = ((zeta_threshold).sqrt());
            let t14 = ((t10).sqrt());
            let t16 = (((t10).simd_le(zeta_threshold)).select(t12 * zeta_threshold, t14 * t10));
            let t17 = f64x8::splat(1.0) / t4 * t16;
            let t18 = f64x8::splat(M_SQRT2);
            let t19 = ((v_rho).sqrt());
            let t20 = t18 * t19;
            let t21 = v_rho * v_rho;
            let t22 = t21 * v_rho;
            let t23 = f64x8::splat(1.0) / t22;
            let t24 = v_sigma * t23;
            let t26 = v_sigma * v_sigma;
            let t27 = t21 * t21;
            let t29 = f64x8::splat(1.0) / t27 / t21;
            let t32 = f64x8::splat(1.0) + f64x8::splat(0.8250592249883855) * t24 + f64x8::splat(0.0025211952768090192) * t26 * t29;
            let t33 = (simd::pow(t32, f64x8::splat(1.0) / f64x8::splat(15.0)));
            let t43 = f64x8::splat(1.0) + f64x8::splat(0.05587702687752028) * t24 + (-f64x8::splat(0.1544) * v_tau / t21 - f64x8::splat(11.596246802930645)) / f64x8::splat(M_PI) / f64x8::splat(4.0);
            let t44 = (simd::pow(t32, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t45 = f64x8::splat(1.0) / t44;
            let t48 = f64x8::splat(1.0) / t33 + f64x8::splat(2.0) / f64x8::splat(5.0) * t43 * t45;
            let t52 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t17 * t20 * t48));
            let tzk0 = f64x8::splat(2.0) * t52;
            acc_zk = tzk0;
            let t54 = t18 / t19;
            let t59 = f64x8::splat(1.0) / t33 / t32;
            let t60 = f64x8::splat(1.0) / t27;
            let t61 = v_sigma * t60;
            let t63 = t27 * t22;
            let t64 = f64x8::splat(1.0) / t63;
            let t67 = -f64x8::splat(2.475177674965156) * t61 - f64x8::splat(0.015127171660854116) * t26 * t64;
            let t73 = -f64x8::splat(0.16763108063256085) * t61 + f64x8::splat(0.02457352321338864) * v_tau * t23;
            let t77 = f64x8::splat(1.0) / t44 / t32;
            let t78 = t43 * t77;
            let t81 = -t59 * t67 / f64x8::splat(15.0) + f64x8::splat(2.0) / f64x8::splat(5.0) * t73 * t45 - f64x8::splat(2.0) / f64x8::splat(25.0) * t78 * t67;
            let t86 = ((t3).select(f64x8::splat(0.0), -t17 * t54 * t48 / f64x8::splat(3.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t17 * t20 * t81));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t86 + f64x8::splat(2.0) * t52;
            acc_vrho = tvrho0;
            let t90 = v_sigma * t29;
            let t92 = f64x8::splat(0.8250592249883855) * t23 + f64x8::splat(0.0050423905536180385) * t90;
            let t99 = -t59 * t92 / f64x8::splat(15.0) + f64x8::splat(0.022350810751008112) * t23 * t45 - f64x8::splat(2.0) / f64x8::splat(25.0) * t78 * t92;
            let t103 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t17 * t20 * t99));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t103;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t105 = t16 * t18;
            let t107 = f64x8::splat(1.0) / t19 / v_rho;
            let t111 = ((t3).select(f64x8::splat(0.0), f64x8::splat(0.0018485501104083812) * t105 * t107 * t45));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t111;
            acc_vtau = tvtau0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(vlapl, ip, m, acc_vlapl);
        store_add(vtau, ip, m, acc_vtau);
        ip += 8;
    }
}
