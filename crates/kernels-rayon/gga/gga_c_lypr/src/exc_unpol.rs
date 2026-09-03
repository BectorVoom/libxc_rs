//! GGA_C_LYPR exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_lypr.c`
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
pub fn gga_c_lypr_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_m1: f64,
    param_omega: f64,
    param_d: f64,
    param_m2: f64,
    param_b: f64,
    param_c: f64,
    param_a: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_m1 = f64x8::splat(param_m1);
    let param_omega = f64x8::splat(param_omega);
    let param_d = f64x8::splat(param_d);
    let param_m2 = f64x8::splat(param_m2);
    let param_b = f64x8::splat(param_b);
    let param_c = f64x8::splat(param_c);
    let param_a = f64x8::splat(param_a);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t2 = (simd::cbrt(v_rho));
            let t3 = f64x8::splat(1.0) / t2;
            let t5 = (simd::erfc(param_m1 * param_omega * t3));
            let t7 = param_d * t3 + f64x8::splat(1.0);
            let t8 = f64x8::splat(1.0) / t7;
            let t10 = param_m2 * param_omega;
            let t12 = (simd::erfc(t10 * t3));
            let t13 = t12 * param_b;
            let t15 = (simd::exp(-param_c * t3));
            let t16 = t15 * t8;
            let t17 = v_rho * v_rho;
            let t18 = t2 * t2;
            let t20 = f64x8::splat(1.0) / t18 / t17;
            let t21 = v_sigma * t20;
            let t23 = param_d * t8 + param_c;
            let t24 = t23 * t3;
            let t26 = -f64x8::splat(1.0) / f64x8::splat(72.0) - f64x8::splat(7.0) / f64x8::splat(72.0) * t24;
            let t28 = f64x8::splat(M_CBRT3);
            let t29 = t28 * t28;
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = t31 * t31;
            let t34 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t35 = zeta_threshold * zeta_threshold;
            let t36 = (simd::cbrt(zeta_threshold));
            let t37 = t36 * t36;
            let t39 = ((t34).select(t37 * t35, f64x8::splat(1.0)));
            let t43 = f64x8::splat(5.0) / f64x8::splat(2.0) - t24 / f64x8::splat(18.0);
            let t44 = t43 * v_sigma;
            let t45 = t20 * t39;
            let t48 = t24 - f64x8::splat(11.0);
            let t49 = t48 * v_sigma;
            let t52 = ((t34).select(t37 * t35 * zeta_threshold, f64x8::splat(1.0)));
            let t53 = t20 * t52;
            let t56 = f64x8::splat(M_CBRT2);
            let t57 = t56 * t56;
            let t58 = v_sigma * t57;
            let t61 = ((t34).select(t35, f64x8::splat(1.0)));
            let t62 = t61 * v_sigma;
            let t64 = t57 * t20 * t39;
            let t70 = -t21 * t26 - f64x8::splat(3.0) / f64x8::splat(10.0) * t29 * t32 * t39 + t44 * t45 / f64x8::splat(8.0) + t49 * t53 / f64x8::splat(144.0) - t56 * (f64x8::splat(4.0) / f64x8::splat(3.0) * t58 * t45 - t62 * t64 / f64x8::splat(2.0)) / f64x8::splat(8.0);
            let t71 = t16 * t70;
            let t73 = param_b * t15;
            let t74 = ((f64x8::splat(M_PI)).sqrt());
            let t75 = f64x8::splat(1.0) / t74;
            let t76 = t8 * t75;
            let t77 = t73 * t76;
            let t78 = param_m2 * param_m2;
            let t79 = param_omega * param_omega;
            let t81 = f64x8::splat(1.0) / t18;
            let t83 = (simd::exp(-t78 * t79 * t81));
            let t84 = t17 * v_rho;
            let t85 = f64x8::splat(1.0) / t84;
            let t86 = t83 * t85;
            let tzk0 = param_a * (-t5 * t8 + t13 * t71 + f64x8::splat(7.0) / f64x8::splat(36.0) * t77 * t10 * t86 * v_sigma);
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
