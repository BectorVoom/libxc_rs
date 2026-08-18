//! LDA_C_HL fxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_hl.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_hl_fxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    param_hl_c_0: f64,
    param_hl_r_0: f64,
    param_hl_c_1: f64,
    param_hl_r_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = param_hl_c_0;
        let t2 = 1.0 / M_PI;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t6 = param_hl_r_0;
        let t7 = t6 * t6;
        let t8 = t7 * t6;
        let t9 = 1.0 / t8;
        let t12 = 1.0 + 3.0 / 4.0 * t5 * t9;
        let t13 = M_CBRT3;
        let t14 = t13 * t13;
        let t15 = pow_1_3(t2);
        let t16 = 1.0 / t15;
        let t17 = t14 * t16;
        let t18 = M_CBRT4;
        let t19 = pow_1_3(t3);
        let t20 = t18 * t19;
        let t24 = 1.0 + t17 * t20 * t6 / 3.0;
        let t25 = f64::ln(t24);
        let t27 = t15 * t15;
        let t28 = t14 * t27;
        let t29 = t19 * t19;
        let t31 = t18 / t29;
        let t32 = 1.0 / t7;
        let t36 = t13 * t15;
        let t37 = t18 * t18;
        let t39 = t37 / t19;
        let t40 = 1.0 / t6;
        let t45 = t1 * (t12 * t25 - t28 * t31 * t32 / 4.0 + t36 * t39 * t40 / 8.0 - 1.0 / 3.0);
        let t46 = rho0 - rho1;
        let t47 = t46 * t4;
        let t48 = 1.0 + t47;
        let t49 = t48 <= zeta_threshold;
        let t50 = pow_1_3(zeta_threshold);
        let t51 = t50 * zeta_threshold;
        let t52 = pow_1_3(t48);
        let t54 = piecewise3(t49, t51, t52 * t48);
        let t55 = 1.0 - t47;
        let t56 = t55 <= zeta_threshold;
        let t57 = pow_1_3(t55);
        let t59 = piecewise3(t56, t51, t57 * t55);
        let t61 = M_CBRT2;
        let t64 = 1.0 / (2.0 * t61 - 2.0);
        let t65 = (t54 + t59 - 2.0) * t64;
        let t66 = param_hl_c_1;
        let t67 = param_hl_r_1;
        let t68 = t67 * t67;
        let t69 = t68 * t67;
        let t70 = 1.0 / t69;
        let t73 = 1.0 + 3.0 / 4.0 * t5 * t70;
        let t77 = 1.0 + t17 * t20 * t67 / 3.0;
        let t78 = f64::ln(t77);
        let t80 = 1.0 / t68;
        let t84 = 1.0 / t67;
        let t90 = -t66 * (t73 * t78 - t28 * t31 * t80 / 4.0 + t36 * t39 * t84 / 8.0 - 1.0 / 3.0) + t45;
        let t91 = t65 * t90;
        let tzk0 = -t45 + t91;
        zk[ip] += tzk0;
        let t92 = t3 * t3;
        let t93 = 1.0 / t92;
        let t94 = t2 * t93;
        let t95 = t9 * t25;
        let t99 = t12 * t14 * t16;
        let t100 = 1.0 / t24;
        let t101 = t6 * t100;
        let t107 = t18 / t29 / t3;
        let t113 = t37 / t19 / t3;
        let t118 = t1 * (-3.0 / 4.0 * t94 * t95 + t99 * t31 * t101 / 9.0 + t28 * t107 * t32 / 6.0 - t36 * t113 * t40 / 24.0);
        let t119 = t46 * t93;
        let t120 = t4 - t119;
        let t123 = piecewise3(t49, 0.0, 4.0 / 3.0 * t52 * t120);
        let t124 = -t120;
        let t127 = piecewise3(t56, 0.0, 4.0 / 3.0 * t57 * t124);
        let t129 = (t123 + t127) * t64;
        let t130 = t129 * t90;
        let t131 = t70 * t78;
        let t135 = t73 * t14 * t16;
        let t136 = 1.0 / t77;
        let t137 = t67 * t136;
        let t149 = -t66 * (-3.0 / 4.0 * t94 * t131 + t135 * t31 * t137 / 9.0 + t28 * t107 * t80 / 6.0 - t36 * t113 * t84 / 24.0) + t118;
        let t150 = t65 * t149;
        let tvrho0 = -t45 + t91 + t3 * (-t118 + t130 + t150);
        vrho[ip * 2] += tvrho0;
        let t153 = -t4 - t119;
        let t156 = piecewise3(t49, 0.0, 4.0 / 3.0 * t52 * t153);
        let t157 = -t153;
        let t160 = piecewise3(t56, 0.0, 4.0 / 3.0 * t57 * t157);
        let t162 = (t156 + t160) * t64;
        let t163 = t162 * t90;
        let tvrho1 = -t45 + t91 + t3 * (-t118 + t163 + t150);
        vrho[ip * 2 + 1] += tvrho1;
        let t166 = 2.0 * t118;
        let t168 = 2.0 * t150;
        let t169 = t92 * t3;
        let t170 = 1.0 / t169;
        let t171 = t2 * t170;
        let t175 = 1.0 / t29 / t92;
        let t176 = t2 * t175;
        let t179 = t17 * t18 * t100;
        let t186 = 1.0 / t27;
        let t187 = t12 * t13 * t186;
        let t188 = t24 * t24;
        let t189 = 1.0 / t188;
        let t190 = t7 * t189;
        let t194 = t18 * t175;
        let t200 = t37 / t19 / t92;
        let t205 = t1 * (3.0 / 2.0 * t171 * t95 - t176 * t32 * t179 / 6.0 - 2.0 / 27.0 * t99 * t107 * t101 - t187 * t113 * t190 / 27.0 - 5.0 / 18.0 * t28 * t194 * t32 + t36 * t200 * t40 / 18.0);
        let t206 = t52 * t52;
        let t207 = 1.0 / t206;
        let t208 = t120 * t120;
        let t211 = t46 * t170;
        let t213 = -2.0 * t93 + 2.0 * t211;
        let t217 = piecewise3(t49, 0.0, 4.0 / 9.0 * t207 * t208 + 4.0 / 3.0 * t52 * t213);
        let t218 = t57 * t57;
        let t219 = 1.0 / t218;
        let t220 = t124 * t124;
        let t223 = -t213;
        let t227 = piecewise3(t56, 0.0, 4.0 / 9.0 * t219 * t220 + 4.0 / 3.0 * t57 * t223);
        let t229 = (t217 + t227) * t64;
        let t230 = t229 * t90;
        let t231 = t129 * t149;
        let t232 = 2.0 * t231;
        let t237 = t17 * t18 * t136;
        let t244 = t73 * t13 * t186;
        let t245 = t77 * t77;
        let t246 = 1.0 / t245;
        let t247 = t68 * t246;
        let t259 = -t66 * (3.0 / 2.0 * t171 * t131 - t176 * t80 * t237 / 6.0 - 2.0 / 27.0 * t135 * t107 * t137 - t244 * t113 * t247 / 27.0 - 5.0 / 18.0 * t28 * t194 * t80 + t36 * t200 * t84 / 18.0) + t205;
        let t260 = t65 * t259;
        let tv2rho20 = -t166 + 2.0 * t130 + t168 + t3 * (-t205 + t230 + t232 + t260);
        v2rho2[ip * 3] += tv2rho20;
        let t263 = t207 * t153;
        let t266 = t52 * t46;
        let t270 = piecewise3(t49, 0.0, 4.0 / 9.0 * t263 * t120 + 8.0 / 3.0 * t266 * t170);
        let t271 = t219 * t157;
        let t274 = t57 * t46;
        let t278 = piecewise3(t56, 0.0, 4.0 / 9.0 * t271 * t124 - 8.0 / 3.0 * t274 * t170);
        let t280 = (t270 + t278) * t64;
        let t281 = t280 * t90;
        let t282 = t162 * t149;
        let tv2rho21 = -t166 + t130 + t168 + t163 + t3 * (-t205 + t281 + t282 + t231 + t260);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t286 = t153 * t153;
        let t290 = 2.0 * t93 + 2.0 * t211;
        let t294 = piecewise3(t49, 0.0, 4.0 / 9.0 * t207 * t286 + 4.0 / 3.0 * t52 * t290);
        let t295 = t157 * t157;
        let t298 = -t290;
        let t302 = piecewise3(t56, 0.0, 4.0 / 9.0 * t219 * t295 + 4.0 / 3.0 * t57 * t298);
        let t304 = (t294 + t302) * t64;
        let t305 = t304 * t90;
        let t306 = 2.0 * t282;
        let tv2rho22 = -t166 + 2.0 * t163 + t168 + t3 * (-t205 + t305 + t306 + t260);
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}
