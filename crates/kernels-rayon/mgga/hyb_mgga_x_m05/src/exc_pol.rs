//! HYB_MGGA_X_M05 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_m05.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn hyb_mgga_x_m05_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_csi_HF: f64,
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
    param_a_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
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
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = t6 * t26;
        let t28 = pow_1_3(t7);
        let t29 = t28 * param_csi_HF;
        let t30 = M_CBRT6;
        let t31 = M_PI * M_PI;
        let t32 = pow_1_3(t31);
        let t33 = t32 * t32;
        let t34 = 1.0 / t33;
        let t35 = t30 * t34;
        let t36 = rho0 * rho0;
        let t37 = pow_1_3(rho0);
        let t38 = t37 * t37;
        let t40 = 1.0 / t38 / t36;
        let t44 = 0.804 + 0.009146457198521547 * t35 * sigma0 * t40;
        let t47 = 1.804 - 0.646416 / t44;
        let t48 = param_a_0;
        let t49 = param_a_1;
        let t50 = t30 * t30;
        let t52 = 3.0 / 10.0 * t50 * t33;
        let t54 = 1.0 / t38 / rho0;
        let t55 = tau0 * t54;
        let t56 = t52 - t55;
        let t57 = t49 * t56;
        let t58 = t52 + t55;
        let t59 = 1.0 / t58;
        let t61 = param_a_2;
        let t62 = t56 * t56;
        let t63 = t61 * t62;
        let t64 = t58 * t58;
        let t65 = 1.0 / t64;
        let t67 = param_a_3;
        let t68 = t62 * t56;
        let t69 = t67 * t68;
        let t70 = t64 * t58;
        let t71 = 1.0 / t70;
        let t73 = param_a_4;
        let t74 = t62 * t62;
        let t75 = t73 * t74;
        let t76 = t64 * t64;
        let t77 = 1.0 / t76;
        let t79 = param_a_5;
        let t80 = t74 * t56;
        let t81 = t79 * t80;
        let t82 = t76 * t58;
        let t83 = 1.0 / t82;
        let t85 = param_a_6;
        let t86 = t74 * t62;
        let t87 = t85 * t86;
        let t88 = t76 * t64;
        let t89 = 1.0 / t88;
        let t91 = param_a_7;
        let t92 = t74 * t68;
        let t93 = t91 * t92;
        let t94 = t76 * t70;
        let t95 = 1.0 / t94;
        let t97 = param_a_8;
        let t98 = t74 * t74;
        let t99 = t97 * t98;
        let t100 = t76 * t76;
        let t101 = 1.0 / t100;
        let t103 = param_a_9;
        let t104 = t98 * t56;
        let t105 = t103 * t104;
        let t107 = 1.0 / t100 / t58;
        let t109 = param_a_10;
        let t110 = t98 * t62;
        let t111 = t109 * t110;
        let t113 = 1.0 / t100 / t64;
        let t115 = param_a_11;
        let t117 = t115 * t98 * t68;
        let t119 = 1.0 / t100 / t70;
        let t121 = t99 * t101 + t105 * t107 + t111 * t113 + t117 * t119 + t57 * t59 + t63 * t65 + t69 * t71 + t75 * t77 + t81 * t83 + t87 * t89 + t93 * t95 + t48;
        let t122 = t47 * t121;
        let t123 = t29 * t122;
        let t126 = piecewise3(t2, 0.0, -3.0 / 8.0 * t27 * t123);
        let t127 = rho1 <= dens_threshold;
        let t128 = -t17;
        let t130 = piecewise5(t15, t12, t11, t16, t128 * t8);
        let t131 = 1.0 + t130;
        let t132 = t131 <= zeta_threshold;
        let t133 = pow_1_3(t131);
        let t135 = piecewise3(t132, t23, t133 * t131);
        let t136 = t6 * t135;
        let t137 = rho1 * rho1;
        let t138 = pow_1_3(rho1);
        let t139 = t138 * t138;
        let t141 = 1.0 / t139 / t137;
        let t145 = 0.804 + 0.009146457198521547 * t35 * sigma2 * t141;
        let t148 = 1.804 - 0.646416 / t145;
        let t150 = 1.0 / t139 / rho1;
        let t151 = tau1 * t150;
        let t152 = t52 - t151;
        let t153 = t49 * t152;
        let t154 = t52 + t151;
        let t155 = 1.0 / t154;
        let t157 = t152 * t152;
        let t158 = t61 * t157;
        let t159 = t154 * t154;
        let t160 = 1.0 / t159;
        let t162 = t157 * t152;
        let t163 = t67 * t162;
        let t164 = t159 * t154;
        let t165 = 1.0 / t164;
        let t167 = t157 * t157;
        let t168 = t73 * t167;
        let t169 = t159 * t159;
        let t170 = 1.0 / t169;
        let t172 = t167 * t152;
        let t173 = t79 * t172;
        let t174 = t169 * t154;
        let t175 = 1.0 / t174;
        let t177 = t167 * t157;
        let t178 = t85 * t177;
        let t179 = t169 * t159;
        let t180 = 1.0 / t179;
        let t182 = t167 * t162;
        let t183 = t91 * t182;
        let t184 = t169 * t164;
        let t185 = 1.0 / t184;
        let t187 = t167 * t167;
        let t188 = t97 * t187;
        let t189 = t169 * t169;
        let t190 = 1.0 / t189;
        let t192 = t187 * t152;
        let t193 = t103 * t192;
        let t195 = 1.0 / t189 / t154;
        let t197 = t187 * t157;
        let t198 = t109 * t197;
        let t200 = 1.0 / t189 / t159;
        let t203 = t115 * t187 * t162;
        let t205 = 1.0 / t189 / t164;
        let t207 = t153 * t155 + t158 * t160 + t163 * t165 + t168 * t170 + t173 * t175 + t178 * t180 + t183 * t185 + t188 * t190 + t193 * t195 + t198 * t200 + t203 * t205 + t48;
        let t208 = t148 * t207;
        let t209 = t29 * t208;
        let t212 = piecewise3(t127, 0.0, -3.0 / 8.0 * t136 * t209);
        let tzk0 = t126 + t212;
        zk[ip] += tzk0;
    }
}
