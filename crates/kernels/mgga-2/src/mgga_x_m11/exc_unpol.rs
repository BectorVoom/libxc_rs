//! MGGA_X_M11 exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 149 shared lines across all orders.
//! Delta: 149 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_m11_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_a_0: f64,
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_a_6: f64,
    param_a_7: f64,
    param_a_8: f64,
    param_a_9: f64,
    param_a_10: f64,
    param_a_11: f64,
    param_b_0: f64,
    param_b_1: f64,
    param_b_2: f64,
    param_b_3: f64,
    param_b_4: f64,
    param_b_5: f64,
    param_b_6: f64,
    param_b_7: f64,
    param_b_8: f64,
    param_b_9: f64,
    param_b_10: f64,
    param_b_11: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (149 lines) ---
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t13 = t12 <= zeta_threshold;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t13, t14 * zeta_threshold, t16 * t12);
        let t19 = t4 / t5 * t18;
        let t20 = pow_1_3(rho[ip]);
        let t21 = pow_1_3(9.0);
        let t22 = t21 * t21;
        let t24 = pow_1_3(1.0 / M_PI);
        let t25 = t24 * t24;
        let t27 = t22 * t25 * param_hyb_omega_0;
        let t30 = piecewise3(t13, t14, t16);
        let t31 = 1.0 / t30;
        let t34 = t27 * t4 / t20 * t31 / 18.0;
        let t35 = 0.135e1 <= t34;
        let t36 = 0.135e1 < t34;
        let t37 = piecewise3(t36, t34, 0.135e1);
        let t38 = t37 * t37;
        let t41 = t38 * t38;
        let t42 = 1.0 / t41;
        let t44 = t41 * t38;
        let t45 = 1.0 / t44;
        let t47 = t41 * t41;
        let t48 = 1.0 / t47;
        let t51 = 1.0 / t47 / t38;
        let t54 = 1.0 / t47 / t41;
        let t57 = 1.0 / t47 / t44;
        let t59 = t47 * t47;
        let t60 = 1.0 / t59;
        let t63 = piecewise3(t36, 0.135e1, t34);
        let t64 = f64::sqrt(M_PI);
        let t65 = 1.0 / t63;
        let t67 = erf_approx(t65 / 2.0);
        let t69 = t63 * t63;
        let t70 = 1.0 / t69;
        let t72 = f64::exp(-t70 / 4.0);
        let t73 = t72 - 1.0;
        let t76 = t72 - 3.0 / 2.0 - 2.0 * t69 * t73;
        let t79 = 2.0 * t63 * t76 + t64 * t67;
        let t83 = piecewise3(t35, 1.0 / t38 / 36.0 - t42 / 960.0 + t45 / 26880.0 - t48 / 829440.0 + t51 / 28385280.0 - t54 / 0.107347968e10 + t57 / 0.445906944e11 - t60 / 0.20214448128e13, 1.0 - 8.0 / 3.0 * t63 * t79);
        let t84 = t20 * t83;
        let t85 = M_CBRT6;
        let t86 = M_PI * M_PI;
        let t87 = pow_1_3(t86);
        let t88 = t87 * t87;
        let t89 = 1.0 / t88;
        let t90 = t85 * t89;
        let t91 = M_CBRT2;
        let t92 = t91 * t91;
        let t93 = sigma[ip] * t92;
        let t94 = rho[ip] * rho[ip];
        let t95 = t20 * t20;
        let t97 = 1.0 / t95 / t94;
        let t99 = t90 * t93 * t97;
        let t101 = 0.804e0 + 0.914625e-2 * t99;
        let t104 = 0.1804e1 - 0.646416e0 / t101;
        let t106 = param_a_1;
        let t107 = t85 * t85;
        let t109 = 3.0 / 10.0 * t107 * t88;
        let t110 = tau[ip] * t92;
        let t112 = 1.0 / t95 / rho[ip];
        let t113 = t110 * t112;
        let t114 = t109 - t113;
        let t115 = t106 * t114;
        let t116 = t109 + t113;
        let t117 = 1.0 / t116;
        let t119 = param_a_2;
        let t120 = t114 * t114;
        let t121 = t119 * t120;
        let t122 = t116 * t116;
        let t123 = 1.0 / t122;
        let t125 = param_a_3;
        let t126 = t120 * t114;
        let t127 = t125 * t126;
        let t128 = t122 * t116;
        let t129 = 1.0 / t128;
        let t131 = param_a_4;
        let t132 = t120 * t120;
        let t133 = t131 * t132;
        let t134 = t122 * t122;
        let t135 = 1.0 / t134;
        let t137 = param_a_5;
        let t138 = t132 * t114;
        let t139 = t137 * t138;
        let t140 = t134 * t116;
        let t141 = 1.0 / t140;
        let t143 = param_a_6;
        let t144 = t132 * t120;
        let t145 = t143 * t144;
        let t146 = t134 * t122;
        let t147 = 1.0 / t146;
        let t149 = param_a_7;
        let t150 = t132 * t126;
        let t151 = t149 * t150;
        let t152 = t134 * t128;
        let t153 = 1.0 / t152;
        let t155 = param_a_8;
        let t156 = t132 * t132;
        let t157 = t155 * t156;
        let t158 = t134 * t134;
        let t159 = 1.0 / t158;
        let t161 = param_a_9;
        let t162 = t156 * t114;
        let t163 = t161 * t162;
        let t165 = 1.0 / t158 / t116;
        let t167 = param_a_10;
        let t168 = t156 * t120;
        let t169 = t167 * t168;
        let t171 = 1.0 / t158 / t122;
        let t173 = param_a_11;
        let t174 = t156 * t126;
        let t175 = t173 * t174;
        let t177 = 1.0 / t158 / t128;
        let t179 = t115 * t117 + t121 * t123 + t127 * t129 + t133 * t135 + t139 * t141 + t145 * t147 + t151 * t153 + t157 * t159 + t163 * t165 + t169 * t171 + t175 * t177 + param_a_0;
        let t182 = f64::exp(-0.93189002206715572255e-2 * t99);
        let t184 = 0.1552e1 - 0.552e0 * t182;
        let t186 = param_b_1;
        let t187 = t186 * t114;
        let t189 = param_b_2;
        let t190 = t189 * t120;
        let t192 = param_b_3;
        let t193 = t192 * t126;
        let t195 = param_b_4;
        let t196 = t195 * t132;
        let t198 = param_b_5;
        let t199 = t198 * t138;
        let t201 = param_b_6;
        let t202 = t201 * t144;
        let t204 = param_b_7;
        let t205 = t204 * t150;
        let t207 = param_b_8;
        let t208 = t207 * t156;
        let t210 = param_b_9;
        let t211 = t210 * t162;
        let t213 = param_b_10;
        let t214 = t213 * t168;
        let t216 = param_b_11;
        let t217 = t216 * t174;
        let t219 = t187 * t117 + t190 * t123 + t193 * t129 + t196 * t135 + t199 * t141 + t202 * t147 + t205 * t153 + t208 * t159 + t211 * t165 + t214 * t171 + t217 * t177 + param_b_0;
        let t221 = t104 * t179 + t184 * t219;
        let t225 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t84 * t221);
        let tzk0 = 2.0 * t225;
        zk[ip] += tzk0;
    }
}
