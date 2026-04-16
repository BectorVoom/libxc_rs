//! MGGA_X_REGTPSS exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_regtpss.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2, pow_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_regtpss_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = 1.0 / rho[ip];
        let t22 = sigma[ip] * t21;
        let t23 = 1.0 / tau[ip];
        let t24 = t22 * t23;
        let t25 = pow_3(t24);
        let t26 = sigma[ip] * sigma[ip];
        let t27 = rho[ip] * rho[ip];
        let t28 = 1.0 / t27;
        let t29 = t26 * t28;
        let t30 = tau[ip] * tau[ip];
        let t31 = 1.0 / t30;
        let t32 = t29 * t31;
        let t34 = 1.0 + t32 / 64.0;
        let t35 = t34 * t34;
        let t36 = 1.0 / t35;
        let t40 = M_CBRT6;
        let t41 = (10.0 / 81.0 + 0.45938270703125e-2 * t25 * t36) * t40;
        let t42 = M_PI * M_PI;
        let t43 = pow_1_3(t42);
        let t44 = t43 * t43;
        let t45 = 1.0 / t44;
        let t46 = t41 * t45;
        let t47 = M_CBRT2;
        let t48 = t47 * t47;
        let t49 = sigma[ip] * t48;
        let t50 = t19 * t19;
        let t52 = 1.0 / t50 / t27;
        let t53 = t49 * t52;
        let t56 = tau[ip] * t48;
        let t58 = 1.0 / t50 / rho[ip];
        let t61 = t56 * t58 - t53 / 8.0;
        let t62 = t61 * t40;
        let t63 = t62 * t45;
        let t65 = 5.0 / 9.0 * t63 - 1.0;
        let t66 = t45 * t65;
        let t69 = 1.0 + 0.22222222222222222222e0 * t62 * t66;
        let t70 = f64::sqrt(t69);
        let t71 = 1.0 / t70;
        let t74 = t40 * t45;
        let t75 = t74 * t53;
        let t76 = t75 / 36.0;
        let t77 = 9.0 / 20.0 * t65 * t71 + t76;
        let t78 = t77 * t77;
        let t81 = t40 * t40;
        let t83 = 1.0 / t43 / t42;
        let t84 = t81 * t83;
        let t85 = t26 * t47;
        let t86 = t27 * t27;
        let t87 = t86 * rho[ip];
        let t89 = 1.0 / t19 / t87;
        let t91 = t84 * t85 * t89;
        let t93 = 162.0 * t32 + 100.0 * t91;
        let t94 = f64::sqrt(t93);
        let t97 = 0.65823568907145082056e-4 * t91;
        let t99 = t26 * sigma[ip];
        let t100 = t86 * t86;
        let t101 = 1.0 / t100;
        let t103 = 0.54088506107080259512e-5 * t99 * t101;
        let t104 = t46 * t53 / 24.0 + 146.0 / 2025.0 * t78 - 73.0 / 97200.0 * t77 * t94 + t97 + 0.20448759451792765188e-2 * t32 + t103;
        let t106 = 1.0 + 0.61346278355378295562e-1 * t75;
        let t107 = t106 * t106;
        let t108 = 1.0 / t107;
        let t110 = 0.804e0 + t104 * t108;
        let t112 = 0.646416e0 / t110;
        let t113 = -t65;
        let t114 = t113 * t113;
        let t115 = t114 * t113;
        let t116 = t61 * t61;
        let t117 = t116 * t81;
        let t118 = t117 * t83;
        let t120 = 1.0 + 0.67148919753086419753e0 * t118;
        let t121 = f64::sqrt(t120);
        let t123 = 1.0 / t121 / t120;
        let t124 = t115 * t123;
        let t126 = f64::exp(-t75 / 8.0);
        let t128 = -0.45e0 + t76;
        let t129 = t128 * t128;
        let t132 = 2592.0 + 25.0 * t91;
        let t133 = f64::sqrt(t132);
        let t136 = 0.29644443963477366255e-1 * t75 + 146.0 / 2025.0 * t129 - 73.0 / 48600.0 * t128 * t133 + t97 + 0.1308720604914736972e0 + t103;
        let t138 = 0.804e0 + t136 * t108;
        let t141 = -0.646416e0 / t138 + t112;
        let t142 = t126 * t141;
        let t144 = 0.1804e1 - t112 + t124 * t142;
        let t148 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t144);
        let tzk0 = 2.0 * t148;
        zk[ip] += tzk0;
    }
}
