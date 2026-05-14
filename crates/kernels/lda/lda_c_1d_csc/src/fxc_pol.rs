//! LDA_C_1D_CSC fxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_1d_csc.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;

/// LDA_C_1D_CSC fxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_1d_csc_fxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    param_ferro_0: f64,
    param_ferro_1: f64,
    param_ferro_2: f64,
    param_ferro_3: f64,
    param_ferro_4: f64,
    param_ferro_5: f64,
    param_ferro_6: f64,
    param_ferro_7: f64,
    param_ferro_8: f64,
    param_ferro_9: f64,
    param_para_0: f64,
    param_para_1: f64,
    param_para_2: f64,
    param_para_3: f64,
    param_para_4: f64,
    param_para_5: f64,
    param_para_6: f64,
    param_para_7: f64,
    param_para_8: f64,
    param_para_9: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t2 = 1.0 / t1;
        let t3 = t2 / 2.0;
        let t4 = param_para_4;
        let t5 = t1 * t1;
        let t6 = 1.0 / t5;
        let t9 = t3 + t4 * t6 / 4.0;
        let t10 = param_para_7;
        let t14 = param_para_9;
        let t15 = f64::powf(t3, t14);
        let t16 = param_para_8 * t15;
        let t17 = 1.0 + t10 * t2 / 2.0 + t16;
        let t18 = f64::ln(t17);
        let t19 = t9 * t18;
        let t22 = param_para_1;
        let t25 = param_para_5;
        let t26 = f64::powf(t3, t25);
        let t27 = param_para_2 * t26;
        let t30 = param_para_6;
        let t31 = f64::powf(t3, t30);
        let t32 = param_para_3 * t31;
        let t34 = t22 * t2 + 2.0 * t27 + 2.0 * t32 + 2.0 * param_para_0;
        let t35 = 1.0 / t34;
        let t36 = t19 * t35;
        let t37 = param_ferro_4;
        let t40 = t3 + t37 * t6 / 4.0;
        let t41 = param_ferro_7;
        let t45 = param_ferro_9;
        let t46 = f64::powf(t3, t45);
        let t47 = param_ferro_8 * t46;
        let t48 = 1.0 + t41 * t2 / 2.0 + t47;
        let t49 = f64::ln(t48);
        let t50 = t40 * t49;
        let t53 = param_ferro_1;
        let t56 = param_ferro_5;
        let t57 = f64::powf(t3, t56);
        let t58 = param_ferro_2 * t57;
        let t61 = param_ferro_6;
        let t62 = f64::powf(t3, t61);
        let t63 = param_ferro_3 * t62;
        let t65 = t53 * t2 + 2.0 * t58 + 2.0 * t63 + 2.0 * param_ferro_0;
        let t66 = 1.0 / t65;
        let t68 = -t50 * t66 + t36;
        let t69 = rho0 - rho1;
        let t70 = t69 * t69;
        let t71 = t68 * t70;
        let t72 = t71 * t6;
        let tzk0 = -t36 + t72;
        zk[ip] += tzk0;
        let t74 = 1.0 / t5 / t1;
        let t77 = -t4 * t74 / 2.0 - t6 / 2.0;
        let t78 = t77 * t18;
        let t79 = t78 * t35;
        let t84 = -t10 * t6 / 2.0 - t16 * t14 * t2;
        let t85 = t9 * t84;
        let t86 = 1.0 / t17;
        let t87 = t86 * t35;
        let t88 = t85 * t87;
        let t89 = t34 * t34;
        let t90 = 1.0 / t89;
        let t98 = -2.0 * t27 * t25 * t2 - 2.0 * t32 * t30 * t2 - t22 * t6;
        let t99 = t90 * t98;
        let t100 = t19 * t99;
        let t103 = -t37 * t74 / 2.0 - t6 / 2.0;
        let t104 = t103 * t49;
        let t110 = -t41 * t6 / 2.0 - t47 * t45 * t2;
        let t111 = t40 * t110;
        let t112 = 1.0 / t48;
        let t113 = t112 * t66;
        let t115 = t65 * t65;
        let t116 = 1.0 / t115;
        let t124 = -2.0 * t58 * t56 * t2 - 2.0 * t63 * t61 * t2 - t53 * t6;
        let t125 = t116 * t124;
        let t127 = -t104 * t66 - t111 * t113 + t50 * t125 - t100 + t79 + t88;
        let t128 = t127 * t70;
        let t129 = t128 * t6;
        let t130 = t68 * t69;
        let t131 = t130 * t6;
        let t132 = 2.0 * t131;
        let t133 = t71 * t74;
        let t134 = 2.0 * t133;
        let tvrho0 = -t36 + t72 + t1 * (-t79 - t88 + t100 + t129 + t132 - t134);
        vrho[ip * 2] += tvrho0;
        let tvrho1 = -t36 + t72 + t1 * (-t79 - t88 + t100 + t129 - t132 - t134);
        vrho[ip * 2 + 1] += tvrho1;
        let t139 = 2.0 * t79;
        let t140 = 2.0 * t88;
        let t141 = 2.0 * t100;
        let t142 = 2.0 * t129;
        let t143 = 4.0 * t131;
        let t144 = 4.0 * t133;
        let t145 = t5 * t5;
        let t146 = 1.0 / t145;
        let t149 = t74 + 3.0 / 2.0 * t4 * t146;
        let t150 = t149 * t18;
        let t151 = t150 * t35;
        let t152 = t77 * t84;
        let t153 = t152 * t87;
        let t154 = 2.0 * t153;
        let t155 = t78 * t99;
        let t156 = 2.0 * t155;
        let t158 = t14 * t14;
        let t163 = t16 * t14 * t6 + t16 * t158 * t6 + t10 * t74;
        let t164 = t9 * t163;
        let t165 = t164 * t87;
        let t166 = t84 * t84;
        let t167 = t9 * t166;
        let t168 = t17 * t17;
        let t169 = 1.0 / t168;
        let t170 = t169 * t35;
        let t171 = t167 * t170;
        let t172 = t86 * t90;
        let t173 = t172 * t98;
        let t174 = t85 * t173;
        let t175 = 2.0 * t174;
        let t177 = 1.0 / t89 / t34;
        let t178 = t98 * t98;
        let t179 = t177 * t178;
        let t180 = t19 * t179;
        let t181 = 2.0 * t180;
        let t183 = t25 * t25;
        let t188 = t30 * t30;
        let t194 = 2.0 * t27 * t183 * t6 + 2.0 * t32 * t188 * t6 + 2.0 * t27 * t25 * t6 + 2.0 * t32 * t30 * t6 + 2.0 * t22 * t74;
        let t195 = t90 * t194;
        let t196 = t19 * t195;
        let t199 = t74 + 3.0 / 2.0 * t37 * t146;
        let t200 = t199 * t49;
        let t202 = t103 * t110;
        let t208 = t45 * t45;
        let t213 = t47 * t208 * t6 + t47 * t45 * t6 + t41 * t74;
        let t214 = t40 * t213;
        let t216 = t110 * t110;
        let t217 = t40 * t216;
        let t218 = t48 * t48;
        let t219 = 1.0 / t218;
        let t220 = t219 * t66;
        let t222 = t112 * t116;
        let t223 = t222 * t124;
        let t227 = 1.0 / t115 / t65;
        let t228 = t124 * t124;
        let t229 = t227 * t228;
        let t233 = t56 * t56;
        let t238 = t61 * t61;
        let t244 = 2.0 * t58 * t233 * t6 + 2.0 * t63 * t238 * t6 + 2.0 * t58 * t56 * t6 + 2.0 * t63 * t61 * t6 + 2.0 * t53 * t74;
        let t245 = t116 * t244;
        let t247 = 2.0 * t104 * t125 + 2.0 * t111 * t223 - 2.0 * t202 * t113 - t214 * t113 - t200 * t66 + t217 * t220 - 2.0 * t50 * t229 + t50 * t245 + t151 + t154 - t156 + t165 - t171 - t175 + t181 - t196;
        let t248 = t247 * t70;
        let t249 = t248 * t6;
        let t250 = t127 * t69;
        let t251 = t250 * t6;
        let t252 = 4.0 * t251;
        let t253 = t128 * t74;
        let t254 = 4.0 * t253;
        let t255 = t68 * t6;
        let t256 = 2.0 * t255;
        let t257 = t130 * t74;
        let t258 = 8.0 * t257;
        let t259 = t71 * t146;
        let t260 = 6.0 * t259;
        let t261 = -t151 - t154 + t156 - t165 + t171 + t175 - t181 + t196 + t249 + t252 - t254 + t256 - t258 + t260;
        let tv2rho20 = t1 * t261 - t139 - t140 + t141 + t142 + t143 - t144;
        v2rho2[ip * 3] += tv2rho20;
        let t263 = -t151 - t154 + t156 - t165 + t171 + t175 - t181 + t196 + t249 - t254 - t256 + t260;
        let tv2rho21 = t1 * t263 - t139 - t140 + t141 + t142 - t144;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t265 = -t151 - t154 + t156 - t165 + t171 + t175 - t181 + t196 + t249 - t252 - t254 + t256 + t258 + t260;
        let tv2rho22 = t1 * t265 - t139 - t140 + t141 + t142 - t143 - t144;
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}
