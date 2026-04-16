//! GGA_C_PBELOC exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_pbeloc.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_pbeloc_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho0 + rho1;
        let t8 = pow_1_3(t7);
        let t11 = t4 * t6 / t8;
        let t13 = 1.0 + 0.53425e-1 * t11;
        let t14 = f64::sqrt(t11);
        let t17 = pow_3_2(t11);
        let t19 = t1 * t1;
        let t20 = t3 * t3;
        let t21 = t19 * t20;
        let t22 = t8 * t8;
        let t25 = t21 * t5 / t22;
        let t27 = 0.379785e1 * t14 + 0.8969e0 * t11 + 0.204775e0 * t17 + 0.123235e0 * t25;
        let t30 = 1.0 + 0.16081979498692535067e2 / t27;
        let t31 = f64::ln(t30);
        let t33 = 0.621814e-1 * t13 * t31;
        let t34 = rho0 - rho1;
        let t35 = t34 * t34;
        let t36 = t35 * t35;
        let t37 = t7 * t7;
        let t38 = t37 * t37;
        let t39 = 1.0 / t38;
        let t40 = t36 * t39;
        let t41 = 1.0 / t7;
        let t42 = t34 * t41;
        let t43 = 1.0 + t42;
        let t44 = t43 <= zeta_threshold;
        let t45 = pow_1_3(zeta_threshold);
        let t46 = t45 * zeta_threshold;
        let t47 = pow_1_3(t43);
        let t48 = t47 * t43;
        let t49 = piecewise3(t44, t46, t48);
        let t50 = 1.0 - t42;
        let t51 = t50 <= zeta_threshold;
        let t52 = pow_1_3(t50);
        let t53 = t52 * t50;
        let t54 = piecewise3(t51, t46, t53);
        let t55 = t49 + t54 - 2.0;
        let t56 = M_CBRT2;
        let t59 = 1.0 / (2.0 * t56 - 2.0);
        let t60 = t55 * t59;
        let t62 = 1.0 + 0.5137e-1 * t11;
        let t67 = 0.705945e1 * t14 + 0.1549425e1 * t11 + 0.420775e0 * t17 + 0.1562925e0 * t25;
        let t70 = 1.0 + 0.32163958997385070134e2 / t67;
        let t71 = f64::ln(t70);
        let t75 = 1.0 + 0.278125e-1 * t11;
        let t80 = 0.51785e1 * t14 + 0.905775e0 * t11 + 0.1100325e0 * t17 + 0.1241775e0 * t25;
        let t83 = 1.0 + 0.29608749977793437516e2 / t80;
        let t84 = f64::ln(t83);
        let t85 = t75 * t84;
        let t87 = -0.310907e-1 * t62 * t71 + t33 - 0.19751673498613801407e-1 * t85;
        let t88 = t60 * t87;
        let t89 = t40 * t88;
        let t91 = 0.19751673498613801407e-1 * t60 * t85;
        let t92 = f64::ln(2.0);
        let t93 = 1.0 - t92;
        let t94 = M_PI * M_PI;
        let t95 = 1.0 / t94;
        let t96 = t93 * t95;
        let t97 = t45 * t45;
        let t98 = t47 * t47;
        let t99 = piecewise3(t44, t97, t98);
        let t100 = t52 * t52;
        let t101 = piecewise3(t51, t97, t100);
        let t103 = t99 / 2.0 + t101 / 2.0;
        let t104 = t103 * t103;
        let t105 = t104 * t103;
        let t107 = sigma0 + 2.0 * sigma1 + sigma2;
        let t109 = 1.0 / t8 / t37;
        let t110 = t107 * t109;
        let t111 = 1.0 / t104;
        let t112 = t56 * t111;
        let t114 = 1.0 / t3;
        let t115 = t19 * t114;
        let t117 = f64::exp(-t25 / 4.0);
        let t118 = 1.0 - t117;
        let t119 = t5 * t118;
        let t120 = t115 * t119;
        let t123 = 0.375e-1 + 0.83333333333333333332e-3 * t110 * t112 * t120;
        let t125 = t111 * t19;
        let t126 = t114 * t5;
        let t127 = t125 * t126;
        let t130 = 1.0 / t93;
        let t131 = t123 * t130;
        let t133 = (-t33 + t89 + t91) * t130;
        let t134 = 1.0 / t105;
        let t135 = t94 * t134;
        let t137 = f64::exp(-t133 * t135);
        let t138 = t137 - 1.0;
        let t139 = 1.0 / t138;
        let t140 = t94 * t139;
        let t141 = t107 * t107;
        let t142 = t140 * t141;
        let t143 = t131 * t142;
        let t145 = 1.0 / t22 / t38;
        let t146 = t56 * t56;
        let t147 = t145 * t146;
        let t148 = t104 * t104;
        let t149 = 1.0 / t148;
        let t151 = 1.0 / t20;
        let t152 = t1 * t151;
        let t153 = t152 * t6;
        let t154 = t147 * t149 * t153;
        let t157 = t110 * t56 * t127 / 96.0 + t143 * t154 / 3072.0;
        let t158 = t123 * t157;
        let t159 = t130 * t94;
        let t160 = t140 * t157;
        let t162 = t131 * t160 + 1.0;
        let t163 = 1.0 / t162;
        let t164 = t159 * t163;
        let t166 = t158 * t164 + 1.0;
        let t167 = f64::ln(t166);
        let t169 = t96 * t105 * t167;
        let tzk0 = -t33 + t89 + t91 + t169;
        zk[ip] += tzk0;
    }
}
