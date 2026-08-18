//! LDA_C_CHACHIYO_MOD fxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_chachiyo_mod.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_chachiyo_mod_fxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    param_af: f64,
    param_ap: f64,
    param_bf: f64,
    param_bp: f64,
    param_cf: f64,
    param_cp: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = M_CBRT3;
        let t2 = t1 * t1;
        let t3 = param_bp * t2;
        let t5 = pow_1_3(1.0 / M_PI);
        let t7 = M_CBRT4;
        let t8 = 1.0 / t5 * t7;
        let t9 = rho0 + rho1;
        let t10 = pow_1_3(t9);
        let t11 = t8 * t10;
        let t14 = param_cp * t1;
        let t15 = t5 * t5;
        let t17 = t7 * t7;
        let t18 = 1.0 / t15 * t17;
        let t19 = t10 * t10;
        let t20 = t18 * t19;
        let t23 = 1.0 + t3 * t11 / 3.0 + t14 * t20 / 3.0;
        let t24 = f64::ln(t23);
        let t25 = param_ap * t24;
        let t26 = param_bf * t2;
        let t29 = param_cf * t1;
        let t32 = 1.0 + t26 * t11 / 3.0 + t29 * t20 / 3.0;
        let t33 = f64::ln(t32);
        let t35 = param_af * t33 - t25;
        let t36 = rho0 - rho1;
        let t37 = 1.0 / t9;
        let t38 = t36 * t37;
        let t39 = 1.0 + t38;
        let t40 = t39 <= zeta_threshold;
        let t41 = pow_1_3(zeta_threshold);
        let t42 = t41 * t41;
        let t43 = pow_1_3(t39);
        let t44 = t43 * t43;
        let t45 = piecewise3(t40, t42, t44);
        let t46 = 1.0 - t38;
        let t47 = t46 <= zeta_threshold;
        let t48 = pow_1_3(t46);
        let t49 = t48 * t48;
        let t50 = piecewise3(t47, t42, t49);
        let t52 = t45 / 2.0 + t50 / 2.0;
        let t53 = t52 * t52;
        let t56 = -2.0 * t53 * t52 + 2.0;
        let t57 = t35 * t56;
        let tzk0 = t25 + t57;
        zk[ip] += tzk0;
        let t59 = t8 / t19;
        let t63 = t18 / t10;
        let t66 = t3 * t59 / 9.0 + 2.0 / 9.0 * t14 * t63;
        let t68 = 1.0 / t23;
        let t69 = param_ap * t66 * t68;
        let t74 = t26 * t59 / 9.0 + 2.0 / 9.0 * t29 * t63;
        let t76 = 1.0 / t32;
        let t78 = param_af * t74 * t76 - t69;
        let t79 = t78 * t56;
        let t80 = t35 * t53;
        let t81 = 1.0 / t43;
        let t82 = t9 * t9;
        let t83 = 1.0 / t82;
        let t84 = t36 * t83;
        let t85 = t37 - t84;
        let t88 = piecewise3(t40, 0.0, 2.0 / 3.0 * t81 * t85);
        let t89 = 1.0 / t48;
        let t90 = -t85;
        let t93 = piecewise3(t47, 0.0, 2.0 / 3.0 * t89 * t90);
        let t95 = t88 / 2.0 + t93 / 2.0;
        let t96 = t80 * t95;
        let t97 = 6.0 * t96;
        let tvrho0 = t25 + t57 + t9 * (t69 + t79 - t97);
        vrho[ip * 2] += tvrho0;
        let t100 = -t37 - t84;
        let t103 = piecewise3(t40, 0.0, 2.0 / 3.0 * t81 * t100);
        let t104 = -t100;
        let t107 = piecewise3(t47, 0.0, 2.0 / 3.0 * t89 * t104);
        let t109 = t103 / 2.0 + t107 / 2.0;
        let t110 = t80 * t109;
        let t111 = 6.0 * t110;
        let tvrho1 = t25 + t57 + t9 * (t69 + t79 - t111);
        vrho[ip * 2 + 1] += tvrho1;
        let t114 = 2.0 * t69;
        let t115 = 2.0 * t79;
        let t119 = t8 / t19 / t9;
        let t123 = t18 / t10 / t9;
        let t126 = -2.0 / 27.0 * t3 * t119 - 2.0 / 27.0 * t14 * t123;
        let t127 = param_ap * t126;
        let t128 = t127 * t68;
        let t129 = t66 * t66;
        let t131 = t23 * t23;
        let t132 = 1.0 / t131;
        let t133 = param_ap * t129 * t132;
        let t137 = -2.0 / 27.0 * t26 * t119 - 2.0 / 27.0 * t29 * t123;
        let t138 = param_af * t137;
        let t140 = t74 * t74;
        let t142 = t32 * t32;
        let t143 = 1.0 / t142;
        let t145 = -param_af * t140 * t143 + t138 * t76 - t128 + t133;
        let t146 = t145 * t56;
        let t147 = t78 * t53;
        let t148 = t147 * t95;
        let t149 = 12.0 * t148;
        let t150 = t35 * t52;
        let t151 = t95 * t95;
        let t152 = t150 * t151;
        let t153 = 12.0 * t152;
        let t155 = 1.0 / t43 / t39;
        let t156 = t85 * t85;
        let t159 = t82 * t9;
        let t160 = 1.0 / t159;
        let t161 = t36 * t160;
        let t163 = -2.0 * t83 + 2.0 * t161;
        let t167 = piecewise3(t40, 0.0, -2.0 / 9.0 * t155 * t156 + 2.0 / 3.0 * t81 * t163);
        let t169 = 1.0 / t48 / t46;
        let t170 = t90 * t90;
        let t173 = -t163;
        let t177 = piecewise3(t47, 0.0, -2.0 / 9.0 * t169 * t170 + 2.0 / 3.0 * t89 * t173);
        let t179 = t167 / 2.0 + t177 / 2.0;
        let t180 = t80 * t179;
        let t181 = 6.0 * t180;
        let tv2rho20 = t114 + t115 - 12.0 * t96 + t9 * (t128 - t133 + t146 - t149 - t153 - t181);
        v2rho2[ip * 3] += tv2rho20;
        let t185 = t147 * t109;
        let t187 = t109 * t95;
        let t188 = t150 * t187;
        let t190 = t155 * t100;
        let t193 = t81 * t36;
        let t197 = piecewise3(t40, 0.0, -2.0 / 9.0 * t190 * t85 + 4.0 / 3.0 * t193 * t160);
        let t198 = t169 * t104;
        let t201 = t89 * t36;
        let t205 = piecewise3(t47, 0.0, -2.0 / 9.0 * t198 * t90 - 4.0 / 3.0 * t201 * t160);
        let t207 = t197 / 2.0 + t205 / 2.0;
        let t208 = t80 * t207;
        let tv2rho21 = t114 + t115 - t97 - t111 + t9 * (t128 - t133 + t146 - 6.0 * t148 - 6.0 * t185 - 12.0 * t188 - 6.0 * t208);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t213 = 12.0 * t185;
        let t214 = t109 * t109;
        let t215 = t150 * t214;
        let t216 = 12.0 * t215;
        let t217 = t100 * t100;
        let t221 = 2.0 * t83 + 2.0 * t161;
        let t225 = piecewise3(t40, 0.0, -2.0 / 9.0 * t155 * t217 + 2.0 / 3.0 * t81 * t221);
        let t226 = t104 * t104;
        let t229 = -t221;
        let t233 = piecewise3(t47, 0.0, -2.0 / 9.0 * t169 * t226 + 2.0 / 3.0 * t89 * t229);
        let t235 = t225 / 2.0 + t233 / 2.0;
        let t236 = t80 * t235;
        let t237 = 6.0 * t236;
        let tv2rho22 = t114 + t115 - 12.0 * t110 + t9 * (t128 - t133 + t146 - t213 - t216 - t237);
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}
