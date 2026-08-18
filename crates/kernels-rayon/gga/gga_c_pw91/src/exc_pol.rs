//! GGA_C_PW91 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_pw91.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_pw91_exc_pol(
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
        let t14 = f64::sqrt(t11);
        let t17 = pow_3_2(t11);
        let t19 = t1 * t1;
        let t20 = t3 * t3;
        let t21 = t19 * t20;
        let t22 = t8 * t8;
        let t25 = t21 * t5 / t22;
        let t27 = 3.79785 * t14 + 0.8969 * t11 + 0.204775 * t17 + 0.123235 * t25;
        let t30 = 1.0 + 16.081824322151103 / t27;
        let t31 = f64::ln(t30);
        let t33 = 0.062182 * t13 * t31;
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
        let t70 = 1.0 + 32.1646831778707 / t67;
        let t71 = f64::ln(t70);
        let t75 = 1.0 + 0.0278125 * t11;
        let t80 = 5.1785 * t14 + 0.905775 * t11 + 0.1100325 * t17 + 0.1241775 * t25;
        let t83 = 1.0 + 29.608574643216677 / t80;
        let t84 = f64::ln(t83);
        let t85 = t75 * t84;
        let t87 = -0.03109 * t62 * t71 + t33 - 0.019751789702565206 * t85;
        let t88 = t60 * t87;
        let t89 = t40 * t88;
        let t91 = 0.019751789702565206 * t60 * t85;
        let t92 = M_PI * M_PI;
        let t93 = pow_1_3(t92);
        let t94 = t93 * t93;
        let t95 = t19 * t94;
        let t96 = t45 * t45;
        let t97 = t47 * t47;
        let t98 = piecewise3(t44, t96, t97);
        let t99 = t52 * t52;
        let t100 = piecewise3(t51, t96, t99);
        let t102 = t98 / 2.0 + t100 / 2.0;
        let t103 = t102 * t102;
        let t104 = t103 * t102;
        let t105 = 1.0 / t93;
        let t106 = t19 * t105;
        let t108 = sigma0 + 2.0 * sigma1 + sigma2;
        let t110 = 1.0 / t8 / t37;
        let t111 = t108 * t110;
        let t112 = t111 * t56;
        let t113 = 1.0 / t103;
        let t115 = 1.0 / t3;
        let t116 = t115 * t5;
        let t117 = t113 * t19 * t116;
        let t120 = -t33 + t89 + t91;
        let t121 = 1.0 / t104;
        let t123 = 1.0 / t94;
        let t124 = t1 * t123;
        let t127 = f64::exp(-128.97460341341235 * t120 * t121 * t124);
        let t128 = t127 - 1.0;
        let t129 = 1.0 / t128;
        let t130 = t105 * t129;
        let t131 = t108 * t108;
        let t133 = 1.0 / t22 / t38;
        let t134 = t131 * t133;
        let t135 = t130 * t134;
        let t136 = t56 * t56;
        let t137 = t103 * t103;
        let t138 = 1.0 / t137;
        let t139 = t136 * t138;
        let t140 = 1.0 / t20;
        let t141 = t140 * t6;
        let t142 = t139 * t141;
        let t145 = t112 * t117 / 96.0 + 0.0027166129655589867 * t135 * t142;
        let t146 = t1 * t105;
        let t147 = t129 * t108;
        let t148 = t146 * t147;
        let t149 = t110 * t56;
        let t150 = t113 * t115;
        let t151 = t150 * t5;
        let t155 = t19 * t123;
        let t156 = t128 * t128;
        let t157 = 1.0 / t156;
        let t158 = t157 * t131;
        let t159 = t155 * t158;
        let t160 = t133 * t136;
        let t161 = t138 * t140;
        let t162 = t161 * t6;
        let t163 = t160 * t162;
        let t166 = 1.0 + 0.08693161489788757 * t148 * t149 * t151 + 0.0075571056687546295 * t159 * t163;
        let t167 = 1.0 / t166;
        let t171 = 1.0 + 2.7818116767324024 * t106 * t145 * t167;
        let t172 = f64::ln(t171);
        let t175 = 0.002584488143490343 * t95 * t104 * t172;
        let t176 = t2 * t93;
        let t179 = 2.568 + 5.8165 * t11 + 0.00184725 * t25;
        let t182 = 1000.0 + 2180.75 * t11 + 118.0 * t25;
        let t183 = 1.0 / t182;
        let t185 = t179 * t183 - 0.0018535714285714286;
        let t186 = t185 * t102;
        let t187 = t186 * t108;
        let t188 = t176 * t187;
        let t190 = pow_1_3(9.0);
        let t191 = t190 * t190;
        let t193 = t2 * t5 * t191 * t3;
        let t195 = 1.0 / t22 / t37;
        let t197 = t108 * t56;
        let t201 = f64::exp(-25.0 / 18.0 * t193 * t195 * t103 * t197);
        let t202 = t116 * t201;
        let t203 = t149 * t202;
        let t205 = t188 * t203 / 2.0;
        let tzk0 = -t33 + t89 + t91 + t175 + t205;
        zk[ip] += tzk0;
    }
}
