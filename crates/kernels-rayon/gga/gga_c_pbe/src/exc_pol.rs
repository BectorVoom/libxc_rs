//! GGA_C_PBE exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_pbe.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_pbe_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_gamma: f64,
    param_BB: f64,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
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
        let t13 = 1.0 + 0.053425 * t11;
        let t14 = rmath::sqrt(t11);
        let t17 = pow_3_2(t11);
        let t19 = t1 * t1;
        let t20 = t3 * t3;
        let t21 = t19 * t20;
        let t22 = t8 * t8;
        let t25 = t21 * t5 / t22;
        let t27 = 3.79785 * t14 + 0.8969 * t11 + 0.204775 * t17 + 0.123235 * t25;
        let t30 = 1.0 + 16.081979498692537 / t27;
        let t31 = rmath::ln(t30);
        let t33 = 0.0621814 * t13 * t31;
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
        let t62 = 1.0 + 0.05137 * t11;
        let t67 = 7.05945 * t14 + 1.549425 * t11 + 0.420775 * t17 + 0.1562925 * t25;
        let t70 = 1.0 + 32.16395899738507 / t67;
        let t71 = rmath::ln(t70);
        let t75 = 1.0 + 0.0278125 * t11;
        let t80 = 5.1785 * t14 + 0.905775 * t11 + 0.1100325 * t17 + 0.1241775 * t25;
        let t83 = 1.0 + 29.608749977793437 / t80;
        let t84 = rmath::ln(t83);
        let t85 = t75 * t84;
        let t87 = -0.0310907 * t62 * t71 + t33 - 0.0197516734986138 * t85;
        let t88 = t60 * t87;
        let t89 = t40 * t88;
        let t91 = 0.0197516734986138 * t60 * t85;
        let t92 = t45 * t45;
        let t93 = t47 * t47;
        let t94 = piecewise3(t44, t92, t93);
        let t95 = t52 * t52;
        let t96 = piecewise3(t51, t92, t95);
        let t98 = t94 / 2.0 + t96 / 2.0;
        let t99 = t98 * t98;
        let t100 = t99 * t98;
        let t101 = param_gamma * t100;
        let t103 = sigma0 + 2.0 * sigma1 + sigma2;
        let t105 = 1.0 / t8 / t37;
        let t106 = t103 * t105;
        let t108 = 1.0 / t99;
        let t110 = 1.0 / t3;
        let t111 = t110 * t5;
        let t112 = t108 * t19 * t111;
        let t115 = param_BB * param_beta;
        let t116 = 1.0 / param_gamma;
        let t118 = (-t33 + t89 + t91) * t116;
        let t119 = 1.0 / t100;
        let t121 = rmath::exp(-t118 * t119);
        let t122 = t121 - 1.0;
        let t123 = 1.0 / t122;
        let t124 = t116 * t123;
        let t125 = t103 * t103;
        let t127 = t115 * t124 * t125;
        let t129 = 1.0 / t22 / t38;
        let t130 = t56 * t56;
        let t131 = t129 * t130;
        let t132 = t99 * t99;
        let t133 = 1.0 / t132;
        let t134 = t131 * t133;
        let t135 = 1.0 / t20;
        let t136 = t1 * t135;
        let t137 = t136 * t6;
        let t138 = t134 * t137;
        let t141 = t106 * t56 * t112 / 96.0 + t127 * t138 / 3072.0;
        let t142 = param_beta * t141;
        let t143 = param_beta * t116;
        let t146 = t143 * t123 * t141 + 1.0;
        let t147 = 1.0 / t146;
        let t148 = t116 * t147;
        let t150 = t142 * t148 + 1.0;
        let t151 = rmath::ln(t150);
        let t152 = t101 * t151;
        let tzk0 = -t33 + t89 + t91 + t152;
        zk[ip] += tzk0;
    }
}
