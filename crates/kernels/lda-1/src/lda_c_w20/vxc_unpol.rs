//! LDA_C_W20 vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 90 shared lines across all orders.
//! Delta: 73 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_W20 vxc -- unpolarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_c_w20_vxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (90 lines) ---
        let t1 = f64::ln(2.0);
        let t2 = 1.0 - t1;
        let t3 = M_PI * M_PI;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t6 = t1 / 6.0;
        let t8 = 1.0 / t2;
        let t12 = f64::exp(-2.0 * (-0.16244537117517982 + t6) * t8 * t3);
        let t13 = M_CBRT3;
        let t14 = t13 * t13;
        let t15 = 1.0 / M_PI;
        let t16 = pow_1_3(t15);
        let t17 = t16 * t16;
        let t18 = t14 * t17;
        let t19 = M_CBRT4;
        let t20 = pow_1_3(rho[ip]);
        let t21 = t20 * t20;
        let t22 = 1.0 / t21;
        let t24 = t18 * t19 * t22;
        let t26 = f64::exp(-t24 / 40000.0);
        let t27 = 1.0 - t26;
        let t28 = M_CBRTPI;
        let t29 = t28 * t28;
        let t31 = pow_1_3(9.0);
        let t32 = 1.0 / t29 * t31;
        let t33 = t19 * t19;
        let t39 = t12 / 2.0;
        let t40 = (-0.9 + 3.0 / 16.0 * t32 * t33) * t8 * t3 + t39;
        let t44 = (-2.0 * t27 * t40 + t12) * t14;
        let t45 = 1.0 / t16;
        let t46 = t45 * t19;
        let t47 = t46 * t20;
        let t50 = t27 * t8;
        let t51 = f64::sqrt(4.0);
        let t52 = t13 * t16;
        let t53 = 1.0 / t20;
        let t55 = t52 * t33 * t53;
        let t56 = f64::sqrt(t55);
        let t58 = 1.0 / t56 / t55;
        let t60 = t50 * t51 * t58;
        let t62 = t31 * t31;
        let t63 = t62 * t19;
        let t64 = t29 * t3;
        let t68 = -3.0 / 40.0 * t63 * t64 * t8 + t39;
        let t72 = (-2.0 * t27 * t68 + t12) * t13;
        let t73 = 1.0 / t17;
        let t74 = t73 * t33;
        let t75 = t74 * t21;
        let t78 = 1.0 + t44 * t47 / 3.0 - 118.43525281307231 * t60 + t72 * t75 / 3.0;
        let t79 = f64::ln(t78);
        let t81 = t5 * t79 / 2.0;
        let t82 = t52 * t33;
        let t83 = t53 * t26;
        let t84 = pow_1_4(4.0);
        let t85 = t84 * t84;
        let t86 = t85 * t84;
        let t87 = pow_1_4(t55);
        let t91 = t26 + 5.0 / 8.0 * t86 * t87 * t55;
        let t92 = 1.0 / t91;
        let t93 = t3 * M_PI;
        let t95 = 1.0 / t28 / t93;
        let t97 = 12.0 * t1;
        let t98 = 7.0 / 6.0 * t3 - t97 - 1.0;
        let t99 = t95 * t98;
        let t100 = t14 * t45;
        let t104 = 1.0 + t100 * t19 * t20 / 3.0;
        let t105 = f64::ln(t104);
        let t109 = -t63 * t99 * t105 / 36.0 - 0.01;
        let t110 = t92 * t109;
        let t113 = t82 * t83 * t110 / 4.0;
        let t118 = f64::exp(-4.0 * (-0.1412623711751798 + t6) * t8 * t3);
        let t119 = M_CBRT2;
        let t127 = t118 / 2.0;
        let t128 = 2.0 * (-0.9 + 3.0 / 16.0 * t32 * t33 * t119) * t8 * t3 + t127;
        let t132 = (-2.0 * t27 * t128 + t118) * t14;
        let t136 = t119 * t119;
        let t141 = -3.0 / 20.0 * t63 * t64 * t136 * t8 + t127;
        let t145 = (-2.0 * t27 * t141 + t118) * t13;
        let t148 = 1.0 + t132 * t47 / 3.0 - 236.87050562614462 * t60 + t145 * t75 / 3.0;
        let t149 = f64::ln(t148);
        let t154 = t136 * t62;
        let t156 = 13.0 / 12.0 * t3 - t97 + 1.0 / 2.0;
        let t157 = t95 * t156;
        let t159 = t154 * t157 * t105;
        let t164 = pow_1_3(zeta_threshold);
        let t166 = piecewise3(1.0 <= zeta_threshold, t164 * zeta_threshold, 1.0);
        let t168 = 2.0 * t166 - 2.0;
        let t172 = 1.0 / (2.0 * t119 - 2.0);
        let t173 = (-t5 * t149 / 4.0 - t52 * t83 * t92 * t159 / 144.0 + t81 - t113) * t168 * t172;
        let tzk0 = -t81 + t113 + t173;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (73 lines) ---
        let t175 = 1.0 / t20 / rho[ip];
        let t176 = t175 * t26;
        let t180 = t46 * t22;
        let t183 = f64::powf(4.0, 1.0 / 6.0);
        let t184 = t183 * t183;
        let t185 = t184 * t184;
        let t186 = t185 * t183;
        let t187 = t18 * t186;
        let t189 = 1.0 / t21 / rho[ip];
        let t190 = t189 * t26;
        let t191 = t8 * t58;
        let t193 = t187 * t190 * t191;
        let t195 = t50 * t183;
        let t196 = 4.0 * t24;
        let t198 = 1.0 / t56 / t196;
        let t199 = t198 * t13;
        let t202 = t195 * t199 * t16 * t175;
        let t204 = 1.0 / rho[ip];
        let t205 = t204 * t26;
        let t208 = t74 * t53;
        let t211 = t82 * t176 * t40 / 30000.0 + t44 * t180 / 9.0 + 0.0019739208802178718 * t193 - 236.87050562614462 * t202 + t205 * t68 / 7500.0 + 2.0 / 9.0 * t72 * t208;
        let t212 = 1.0 / t78;
        let t214 = t5 * t211 * t212;
        let t215 = t214 / 2.0;
        let t217 = t82 * t176 * t110;
        let t218 = t217 / 12.0;
        let t219 = rho[ip] * rho[ip];
        let t220 = 1.0 / t219;
        let t221 = t15 * t220;
        let t222 = t26 * t92;
        let t223 = t222 * t109;
        let t224 = t221 * t223;
        let t225 = t224 / 20000.0;
        let t226 = t91 * t91;
        let t227 = 1.0 / t226;
        let t228 = t26 * t227;
        let t229 = t19 * t189;
        let t233 = f64::powf(4.0, 1.0 / 12.0);
        let t234 = t233 * t233;
        let t235 = t234 * t234;
        let t236 = t235 * t233;
        let t237 = t236 * t87;
        let t238 = t52 * t175;
        let t241 = t18 * t229 * t26 / 60000.0 - 25.0 / 24.0 * t237 * t238;
        let t242 = t109 * t241;
        let t243 = t228 * t242;
        let t244 = t55 * t243;
        let t245 = t244 / 4.0;
        let t246 = t19 * t204;
        let t248 = t62 * t95;
        let t249 = 1.0 / t104;
        let t250 = t98 * t249;
        let t251 = t248 * t250;
        let t252 = t246 * t222 * t251;
        let t253 = t252 / 108.0;
        let t265 = t82 * t176 * t128 / 30000.0 + t132 * t180 / 9.0 + 0.0039478417604357436 * t193 - 473.74101125228924 * t202 + t205 * t141 / 7500.0 + 2.0 / 9.0 * t145 * t208;
        let t266 = 1.0 / t148;
        let t274 = t3 * t3;
        let t276 = 1.0 / t28 / t274;
        let t277 = t276 * t220;
        let t278 = t19 * t26;
        let t280 = t92 * t136;
        let t281 = t62 * t156;
        let t282 = t281 * t105;
        let t283 = t280 * t282;
        let t287 = t52 * t83 * t227;
        let t288 = t154 * t95;
        let t289 = t156 * t105;
        let t290 = t289 * t241;
        let t291 = t288 * t290;
        let t297 = t248 * t156 * t19 * t249;
        let t302 = (-t5 * t265 * t266 / 4.0 + t52 * t176 * t92 * t159 / 432.0 - t277 * t278 * t283 / 2880000.0 + t287 * t291 / 144.0 - t205 * t280 * t297 / 432.0 + t215 + t218 - t225 + t245 + t253) * t168 * t172;
        let tvrho0 = -t81 + t113 + t173 + rho[ip] * (-t215 - t218 + t225 - t245 - t253 + t302);
        vrho[ip] += tvrho0;
    }
}
