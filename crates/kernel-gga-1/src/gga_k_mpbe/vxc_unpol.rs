//! GGA_K_MPBE vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_mpbe.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_mpbe_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_a: f64,
    param_c1: f64,
    param_c2: f64,
    param_c3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3(rho[ip]);
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t24 = M_CBRT6;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3(t26);
        let t28 = t27 * t27;
        let t29 = 1.0 / t28;
        let t30 = param_c1 * t24 * t29;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t22 / t34;
        let t37 = param_a * t24;
        let t42 = 1.0 + t37 * t29 * t33 * t36 / 24.0;
        let t43 = 1.0 / t42;
        let t48 = t24 * t24;
        let t51 = 1.0 / t27 / t26;
        let t52 = param_c2 * t48 * t51;
        let t53 = sigma[ip] * sigma[ip];
        let t54 = t53 * t31;
        let t55 = t34 * t34;
        let t56 = t55 * rho[ip];
        let t58 = 1.0 / t21 / t56;
        let t59 = t42 * t42;
        let t60 = 1.0 / t59;
        let t61 = t58 * t60;
        let t65 = t26 * t26;
        let t66 = 1.0 / t65;
        let t67 = param_c3 * t66;
        let t68 = t53 * sigma[ip];
        let t69 = t55 * t55;
        let t70 = 1.0 / t69;
        let t72 = t59 * t42;
        let t73 = 1.0 / t72;
        let t77 = 1.0 + t30 * t33 * t36 * t43 / 24.0 + t52 * t54 * t61 / 288.0 + t67 * t68 * t70 * t73 / 576.0;
        let t81 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t77);
        let tzk0 = 2.0 * t81;
        zk[ip] += tzk0;
        let t83 = t20 / t21;
        let t87 = t34 * rho[ip];
        let t89 = 1.0 / t22 / t87;
        let t94 = param_c1 * t48;
        let t96 = t94 * t51 * t53;
        let t97 = t55 * t34;
        let t99 = 1.0 / t21 / t97;
        let t100 = t31 * t99;
        let t101 = t60 * param_a;
        let t102 = t100 * t101;
        let t105 = t99 * t60;
        let t109 = param_c2 * t66;
        let t110 = t109 * t68;
        let t111 = t69 * rho[ip];
        let t112 = 1.0 / t111;
        let t113 = t112 * t73;
        let t114 = t113 * param_a;
        let t121 = t53 * t53;
        let t122 = t69 * t87;
        let t124 = 1.0 / t22 / t122;
        let t127 = t59 * t59;
        let t128 = 1.0 / t127;
        let t131 = t24 * t29 * t32;
        let t132 = t128 * param_a * t131;
        let t135 = -t30 * t33 * t89 * t43 / 9.0 + t96 * t102 / 108.0 - t52 * t54 * t105 / 54.0 + t110 * t114 / 108.0 - t67 * t68 * t112 * t73 / 72.0 + t67 * t121 * t124 * t132 / 1728.0;
        let t140 = piecewise3(t2, 0.0, t7 * t83 * t77 / 10.0 + 3.0 / 20.0 * t7 * t23 * t135);
        let tvrho0 = 2.0 * rho[ip] * t140 + 2.0 * t81;
        vrho[ip] += tvrho0;
        let t149 = t31 * t58;
        let t150 = t149 * t101;
        let t153 = sigma[ip] * t31;
        let t157 = t109 * t53;
        let t158 = t70 * t73;
        let t159 = t158 * param_a;
        let t166 = t69 * t34;
        let t168 = 1.0 / t22 / t166;
        let t173 = t30 * t32 * t36 * t43 / 24.0 - t94 * t51 * sigma[ip] * t150 / 288.0 + t52 * t153 * t61 / 144.0 - t157 * t159 / 288.0 + t67 * t53 * t70 * t73 / 192.0 - t67 * t68 * t168 * t132 / 4608.0;
        let t177 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t173);
        let tvsigma0 = 2.0 * rho[ip] * t177;
        vsigma[ip] += tvsigma0;
    }
}
