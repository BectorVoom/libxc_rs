//! GGA_X_S12 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_s12.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_s12_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_A: f64,
    param_B: f64,
    param_C: f64,
    param_D: f64,
    param_E: f64,
    param_bx: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = t5 * t25;
        let t27 = pow_1_3(t6);
        let t28 = t27 * param_bx;
        let t29 = param_C * sigma0;
        let t30 = rho0 * rho0;
        let t31 = pow_1_3(rho0);
        let t32 = t31 * t31;
        let t34 = 1.0 / t32 / t30;
        let t36 = sigma0 * sigma0;
        let t37 = param_D * t36;
        let t38 = t30 * t30;
        let t39 = t38 * rho0;
        let t41 = 1.0 / t31 / t39;
        let t43 = t29 * t34 + t37 * t41 + 1.0;
        let t46 = param_B * (1.0 - 1.0 / t43);
        let t47 = param_E * sigma0;
        let t49 = t47 * t34 + 1.0;
        let t51 = 1.0 - 1.0 / t49;
        let t53 = t46 * t51 + param_A;
        let t54 = t28 * t53;
        let t57 = piecewise3(t1, 0.0, -3.0 / 8.0 * t26 * t54);
        let t58 = rho1 <= dens_threshold;
        let t59 = -t16;
        let t61 = piecewise5(t14, t11, t10, t15, t59 * t7);
        let t62 = 1.0 + t61;
        let t63 = t62 <= zeta_threshold;
        let t64 = pow_1_3(t62);
        let t66 = piecewise3(t63, t22, t64 * t62);
        let t67 = t5 * t66;
        let t68 = param_C * sigma2;
        let t69 = rho1 * rho1;
        let t70 = pow_1_3(rho1);
        let t71 = t70 * t70;
        let t73 = 1.0 / t71 / t69;
        let t75 = sigma2 * sigma2;
        let t76 = param_D * t75;
        let t77 = t69 * t69;
        let t78 = t77 * rho1;
        let t80 = 1.0 / t70 / t78;
        let t82 = t68 * t73 + t76 * t80 + 1.0;
        let t85 = param_B * (1.0 - 1.0 / t82);
        let t86 = param_E * sigma2;
        let t88 = t86 * t73 + 1.0;
        let t90 = 1.0 - 1.0 / t88;
        let t92 = t85 * t90 + param_A;
        let t93 = t28 * t92;
        let t96 = piecewise3(t58, 0.0, -3.0 / 8.0 * t67 * t93);
        let tzk0 = t57 + t96;
        zk[ip] += tzk0;
    }
}
