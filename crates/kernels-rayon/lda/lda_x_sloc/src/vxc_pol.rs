//! LDA_X_SLOC vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_sloc.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::piecewise::{piecewise3};

/// LDA_X_SLOC vxc -- polarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_x_sloc_vxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = param_b + 1.0;
        let t3 = 1.0 / t1 / 2.0;
        let t4 = param_a * t3;
        let t5 = rho0 + rho1;
        let t6 = f64::powf(t5, param_b);
        let t7 = rho0 - rho1;
        let t8 = 1.0 / t5;
        let t9 = t7 * t8;
        let t10 = 1.0 + t9;
        let t11 = t10 <= zeta_threshold;
        let t12 = f64::powf(zeta_threshold, t1);
        let t13 = f64::powf(t10, t1);
        let t14 = piecewise3(t11, t12, t13);
        let t15 = 1.0 - t9;
        let t16 = t15 <= zeta_threshold;
        let t17 = f64::powf(t15, t1);
        let t18 = piecewise3(t16, t12, t17);
        let t19 = t14 + t18;
        let tzk0 = -t4 * t6 * t19;
        zk[ip] += tzk0;
        let t22 = t6 * param_b;
        let t24 = t4 * t22 * t19;
        let t25 = t5 * param_a;
        let t26 = t3 * t6;
        let t27 = t13 * t1;
        let t28 = t5 * t5;
        let t29 = 1.0 / t28;
        let t30 = t7 * t29;
        let t31 = t8 - t30;
        let t32 = 1.0 / t10;
        let t35 = piecewise3(t11, 0.0, t27 * t31 * t32);
        let t36 = t17 * t1;
        let t37 = -t31;
        let t38 = 1.0 / t15;
        let t41 = piecewise3(t16, 0.0, t36 * t37 * t38);
        let t42 = t35 + t41;
        let tvrho0 = -t25 * t26 * t42 - t24 + tzk0;
        vrho[ip * 2] += tvrho0;
        let t45 = -t8 - t30;
        let t48 = piecewise3(t11, 0.0, t27 * t45 * t32);
        let t49 = -t45;
        let t52 = piecewise3(t16, 0.0, t36 * t49 * t38);
        let t53 = t48 + t52;
        let tvrho1 = -t25 * t26 * t53 - t24 + tzk0;
        vrho[ip * 2 + 1] += tvrho1;
    }
}
