//! GGA_C_LM exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_lm.c`
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
pub fn gga_c_lm_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_lm_f: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_lm_f = f64x8::splat(param_lm_f);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t1 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t2 = f64x8::splat(1.0) / v_rho;
            let t5 = f64x8::splat(1.0) + t1 * t2 / f64x8::splat(36000.0);
            let t6 = f64x8::splat(M_CBRT3);
            let t7 = t6 * t6;
            let t8 = (simd::cbrt(t1));
            let t9 = f64x8::splat(1.0) / t8;
            let t10 = t7 * t9;
            let t11 = f64x8::splat(M_CBRT4);
            let t12 = (simd::cbrt(v_rho));
            let t14 = t10 * t11 * t12;
            let t16 = f64x8::splat(1.0) + f64x8::splat(10.0) * t14;
            let t17 = (simd::ln(t16));
            let t19 = f64x8::splat(0.0252) * t5 * t17;
            let t20 = t8 * t8;
            let t21 = t7 * t20;
            let t22 = t12 * t12;
            let t23 = f64x8::splat(1.0) / t22;
            let t24 = t11 * t23;
            let t25 = t21 * t24;
            let t26 = f64x8::splat(7e-06) * t25;
            let t27 = t6 * t8;
            let t28 = t11 * t11;
            let t31 = t27 * t28 / t12;
            let t32 = f64x8::splat(0.000105) * t31;
            let t33 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t34 = (simd::cbrt(zeta_threshold));
            let t36 = ((t33).select(t34 * zeta_threshold, f64x8::splat(1.0)));
            let t39 = f64x8::splat(M_CBRT2);
            let t43 = (f64x8::splat(2.0) * t36 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t39 - f64x8::splat(2.0));
            let t45 = f64x8::splat(1.0) + f64x8::splat(5.658842421045167e-07) * t2;
            let t47 = f64x8::splat(1.0) + f64x8::splat(25.0) * t14;
            let t48 = (simd::ln(t47));
            let t54 = t43 * (-f64x8::splat(0.0127) * t45 * t48 - f64x8::splat(6.435555555555556e-06) * t25 + f64x8::splat(8.383333333333333e-05) * t31 - f64x8::splat(0.004166666666666667) + t19);
            let t55 = f64x8::splat(M_PI) * t7;
            let t56 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t57 = (simd::cbrt(t56));
            let t59 = f64x8::splat(1.0) / t57 / t56;
            let t60 = v_rho * v_rho;
            let t62 = f64x8::splat(1.0) / t22 / t60;
            let t63 = v_sigma * t62;
            let t66 = t34 * t34;
            let t68 = ((t33).select(t66 * zeta_threshold, f64x8::splat(1.0)));
            let t69 = ((t68).sqrt());
            let t70 = f64x8::splat(1.0) / t69;
            let t72 = (simd::pow(t1, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t73 = f64x8::splat(1.0) / t72;
            let t74 = ((v_sigma).sqrt());
            let t75 = t73 * t74;
            let t76 = (simd::pow(v_rho, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t81 = (simd::exp(-t6 * param_lm_f * t75 / t76 / v_rho));
            let t82 = t70 * t81;
            let t86 = t59 * (-f64x8::splat(7.0) / f64x8::splat(9.0) * t63 * t36 + f64x8::splat(2.0) * t82 * t63);
            let t89 = t55 * t86 * t12 / f64x8::splat(144.0);
            let tzk0 = -t19 + t26 - t32 + f64x8::splat(0.0084) + t54 + t89;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
