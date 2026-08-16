//! GGA_K_PW86 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_pw86.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_pw86_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
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
        let t3 = t2 * t2;
        let t4 = M_CBRTPI;
        let t6 = t3 * t4 * M_PI;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * t22;
        let t24 = t23 * zeta_threshold;
        let t25 = pow_1_3(t20);
        let t26 = t25 * t25;
        let t28 = piecewise3(t21, t24, t26 * t20);
        let t29 = pow_1_3(t7);
        let t30 = t29 * t29;
        let t31 = t28 * t30;
        let t32 = M_CBRT6;
        let t33 = M_PI * M_PI;
        let t34 = pow_1_3(t33);
        let t35 = t34 * t34;
        let t37 = t32 / t35;
        let t38 = rho0 * rho0;
        let t39 = pow_1_3(rho0);
        let t40 = t39 * t39;
        let t42 = 1.0 / t40 / t38;
        let t46 = t32 * t32;
        let t49 = t46 / t34 / t33;
        let t50 = sigma0 * sigma0;
        let t51 = t38 * t38;
        let t52 = t51 * rho0;
        let t54 = 1.0 / t39 / t52;
        let t58 = t50 * sigma0;
        let t59 = t51 * t51;
        let t60 = 1.0 / t59;
        let t63 = 1.0 + 0.91999999999999999998e-1 * t37 * sigma0 * t42 + 0.1609375e-1 * t49 * t50 * t54 + 0.89114429294134854068e-6 * t58 * t60;
        let t64 = f64::powf(t63, 1.0 / 15.0);
        let t68 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t64);
        let t69 = rho1 <= dens_threshold;
        let t70 = -t17;
        let t72 = piecewise5(t15, t12, t11, t16, t70 * t8);
        let t73 = 1.0 + t72;
        let t74 = t73 <= zeta_threshold;
        let t75 = pow_1_3(t73);
        let t76 = t75 * t75;
        let t78 = piecewise3(t74, t24, t76 * t73);
        let t79 = t78 * t30;
        let t80 = rho1 * rho1;
        let t81 = pow_1_3(rho1);
        let t82 = t81 * t81;
        let t84 = 1.0 / t82 / t80;
        let t88 = sigma2 * sigma2;
        let t89 = t80 * t80;
        let t90 = t89 * rho1;
        let t92 = 1.0 / t81 / t90;
        let t96 = t88 * sigma2;
        let t97 = t89 * t89;
        let t98 = 1.0 / t97;
        let t101 = 1.0 + 0.91999999999999999998e-1 * t37 * sigma2 * t84 + 0.1609375e-1 * t49 * t88 * t92 + 0.89114429294134854068e-6 * t96 * t98;
        let t102 = f64::powf(t101, 1.0 / 15.0);
        let t106 = piecewise3(t69, 0.0, 3.0 / 20.0 * t6 * t79 * t102);
        let tzk0 = t68 + t106;
        zk[ip] += tzk0;
    }
}
