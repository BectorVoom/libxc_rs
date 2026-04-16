//! LDA_C_PMGB06 exc unpol kernel.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_PMGB06 exc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_pmgb06_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = 1.0 <= zeta_threshold;
        let t2 = pow_1_3(zeta_threshold);
        let t3 = t2 * t2;
        let t4 = piecewise3(t1, t3, 1.0);
        let t5 = t4 * t4;
        let t6 = t5 * t4;
        let t7 = f64::ln(2.0);
        let t8 = t7 - 1.0;
        let t10 = 2.0 * t6 * t8;
        let t11 = M_PI * M_PI;
        let t12 = 1.0 / t11;
        let t13 = M_CBRT3;
        let t14 = 1.0 / M_PI;
        let t15 = pow_1_3(t14);
        let t16 = t13 * t15;
        let t17 = M_CBRT4;
        let t18 = t17 * t17;
        let t19 = pow_1_3(rho[ip]);
        let t20 = 1.0 / t19;
        let t21 = t18 * t20;
        let t22 = t16 * t21;
        let t23 = f64::sqrt(t22);
        let t25 = 1.0 / t4;
        let t27 = 2.923025 * param_hyb_omega_0 * t23 * t25;
        let t29 = pow_1_3(9.0);
        let t30 = t29 * t29;
        let t38 = param_hyb_omega_0 * param_hyb_omega_0;
        let t40 = (3.44851 - M_PI * t17 * t30 * t15 / t8 / 12.0) * t38 * t13;
        let t41 = t15 * t18;
        let t42 = 1.0 / t5;
        let t47 = t38 * param_hyb_omega_0;
        let t48 = t23 * t22;
        let t50 = 1.0 / t6;
        let t53 = 1.0 + t27 + t40 * t41 * t20 * t42 / 4.0 + 0.48968 * t47 * t48 * t50;
        let t55 = t38 * t13 * t15;
        let t59 = 1.0 + t27 + 0.8621275 * t55 * t21 * t42;
        let t60 = 1.0 / t59;
        let t62 = f64::ln(t53 * t60);
        let t65 = 1.0 / rho[ip];
        let t74 = (2.0 / 45.0 * t17 * t30 * t15 * (t11 + 6.0 * t7 - 3.0) * t14 - 0.7524) * t13;
        let t78 = t13 * t13;
        let t79 = t15 * t15;
        let t80 = t78 * t79;
        let t81 = t19 * t19;
        let t82 = 1.0 / t81;
        let t83 = t17 * t82;
        let t84 = t80 * t83;
        let t87 = t15 * t14;
        let t88 = t13 * t87;
        let t90 = 1.0 / t19 / rho[ip];
        let t91 = t18 * t90;
        let t94 = 1.0 - t74 * t41 * t20 / 4.0 + 0.0204825 * t84 - 0.0030486129349252553 * t65 + 0.0003485625 * t88 * t91;
        let t97 = f64::exp(-0.1881 * t22);
        let t98 = M_SQRT2;
        let t99 = t97 * t98;
        let t103 = t78 * t79 * t12;
        let t104 = t103 * t17;
        let t106 = 1.0 / t81 / rho[ip];
        let t107 = zeta_threshold * zeta_threshold;
        let t108 = piecewise3(t1, t107, 1.0);
        let t109 = t108 * t30;
        let t110 = 1.0 / t87;
        let t111 = t109 * t110;
        let t113 = M_CBRT2;
        let t115 = t16 * t21 * t113;
        let t117 = 1.0 - 0.0056675 * t115;
        let t119 = t113 * t113;
        let t123 = 1.0 + 0.107975 * t115 + 0.01 * t80 * t83 * t119;
        let t124 = 1.0 / t123;
        let t125 = t117 * t124;
        let t128 = t111 * t13 * t81 * t125 / 15.0;
        let t131 = -1.2375 * t22 + t84 / 4.0;
        let t133 = f64::exp(-0.0775 * t22);
        let t134 = t131 * t133;
        let t135 = M_PI * rho[ip];
        let t138 = t128 + 4.0 / 3.0 * t134 * t135;
        let t145 = t94 * t97;
        let t147 = t145 / 2.0 - 1.0 / 2.0;
        let t150 = t17 * t106;
        let t153 = -0.097 * t22 + 0.169 * t84;
        let t155 = f64::exp(-0.13675 * t22);
        let t157 = t153 * t155 * t13;
        let t159 = 1.0 / t79 * t18;
        let t160 = t159 * t81;
        let t164 = piecewise3(t1, t3 * t107, 1.0);
        let t165 = t164 * t30;
        let t166 = t110 * t13;
        let t170 = t128 + t157 * t160 / 3.0 - t165 * t166 * t81 / 15.0;
        let t175 = 1.0 + 0.053425 * t22;
        let t178 = pow_3_2(t22);
        let t181 = 3.79785 * t23 + 0.8969 * t22 + 0.204775 * t178 + 0.123235 * t84;
        let t184 = 1.0 + 16.081979498692537 / t181;
        let t185 = f64::ln(t184);
        let t189 = piecewise3(t1, t2 * zeta_threshold, 1.0);
        let t195 = (2.0 * t189 - 2.0) / (2.0 * t113 - 2.0);
        let t197 = 1.0 + 0.0278125 * t22;
        let t202 = 5.1785 * t23 + 0.905775 * t22 + 0.1100325 * t178 + 0.1241775 * t84;
        let t205 = 1.0 + 29.608749977793437 / t202;
        let t206 = f64::ln(t205);
        let t210 = -0.0621814 * t175 * t185 + 0.0197516734986138 * t195 * t197 * t206;
        let t215 = t38 * t38;
        let t217 = t103 * t150;
        let t218 = t215 * param_hyb_omega_0;
        let t219 = t98 * t218;
        let t220 = t145 * t219;
        let t226 = rho[ip] * rho[ip];
        let t227 = 1.0 / t226;
        let t231 = t215 * t38;
        let t234 = 1.0 / t81 / t226;
        let t236 = t215 * t215;
        let t240 = t10 * t12 * t62 + (-0.031505407223141116 * t65 * t94 * t99 - 0.005388405304614574 * t104 * t106 * t138 * t98) * t47 + (-0.0837628205355044 * t65 * t147 - 0.011938374665504766 * t103 * t150 * t170 + 0.42708890021612717 * t88 * t91 * t210) * t215 - 0.01197423401025461 * t217 * t220 + (-0.031835665774679375 * t103 * t150 * t147 + 0.05332506774217938 * t227 * t210) * t231 + 0.020267214298646783 * t104 * t234 * t210 * t236;
        let t244 = 1.0 + 0.15403623315025 * t80 * t83 * t38;
        let t245 = t244 * t244;
        let t246 = t245 * t245;
        let t247 = 1.0 / t246;
        let tzk0 = t240 * t247;
        zk[ip] += tzk0;
    }
}
