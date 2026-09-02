//! LDA_X_SLOC kxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_sloc.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_x_sloc_kxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = param_b + 1.0;
        let t3 = 1.0 / t1 / 2.0;
        let t4 = param_a * t3;
        let t5 = rho0 + rho1;
        let t6 = rmath::pow(t5, param_b);
        let t7 = rho0 - rho1;
        let t8 = 1.0 / t5;
        let t9 = t7 * t8;
        let t10 = 1.0 + t9;
        let t11 = t10 <= zeta_threshold;
        let t12 = rmath::pow(zeta_threshold, t1);
        let t13 = rmath::pow(t10, t1);
        let t14 = piecewise3(t11, t12, t13);
        let t15 = 1.0 - t9;
        let t16 = t15 <= zeta_threshold;
        let t17 = rmath::pow(t15, t1);
        let t18 = piecewise3(t16, t12, t17);
        let t19 = t14 + t18;
        let tzk0 = -t4 * t6 * t19;
        zk[ip] += tzk0;
        let t22 = t6 * param_b;
        let t24 = t4 * t22 * t19;
        let t25 = t5 * param_a;
        let t26 = t3 * t6;
        let t27 = t13 * t1;
        let t28 = t5 * t5;
        let t29 = 1.0 / t28;
        let t30 = t7 * t29;
        let t31 = t8 - t30;
        let t32 = 1.0 / t10;
        let t35 = piecewise3(t11, 0.0, t27 * t31 * t32);
        let t36 = t17 * t1;
        let t37 = -t31;
        let t38 = 1.0 / t15;
        let t41 = piecewise3(t16, 0.0, t36 * t37 * t38);
        let t42 = t35 + t41;
        let tvrho0 = -t25 * t26 * t42 - t24 + tzk0;
        vrho[ip * 2] += tvrho0;
        let t45 = -t8 - t30;
        let t48 = piecewise3(t11, 0.0, t27 * t45 * t32);
        let t49 = -t45;
        let t52 = piecewise3(t16, 0.0, t36 * t49 * t38);
        let t53 = t48 + t52;
        let tvrho1 = -t25 * t26 * t53 - t24 + tzk0;
        vrho[ip * 2 + 1] += tvrho1;
        let t56 = t4 * t6;
        let t57 = param_b * t8;
        let t59 = t56 * t57 * t19;
        let t61 = t4 * t6 * t42;
        let t63 = param_b * param_b;
        let t64 = t63 * t8;
        let t66 = t56 * t64 * t19;
        let t68 = t4 * t22 * t42;
        let t70 = t1 * t1;
        let t71 = t13 * t70;
        let t72 = t31 * t31;
        let t73 = t10 * t10;
        let t74 = 1.0 / t73;
        let t75 = t72 * t74;
        let t78 = 1.0 / t28 / t5;
        let t79 = t7 * t78;
        let t81 = -2.0 * t29 + 2.0 * t79;
        let t86 = piecewise3(t11, 0.0, t27 * t81 * t32 - t27 * t75 + t71 * t75);
        let t87 = t17 * t70;
        let t88 = t37 * t37;
        let t89 = t15 * t15;
        let t90 = 1.0 / t89;
        let t91 = t88 * t90;
        let t93 = -t81;
        let t98 = piecewise3(t16, 0.0, t36 * t93 * t38 - t36 * t91 + t87 * t91);
        let t99 = t86 + t98;
        let tv2rho20 = -t25 * t26 * t99 - t59 - 2.0 * t61 - t66 - 2.0 * t68;
        v2rho2[ip * 3] += tv2rho20;
        let t103 = t4 * t6 * t53;
        let t105 = t4 * t22 * t53;
        let t106 = t31 * t74;
        let t107 = t106 * t45;
        let t114 = piecewise3(t11, 0.0, 2.0 * t27 * t79 * t32 - t27 * t107 + t71 * t107);
        let t115 = t37 * t90;
        let t116 = t115 * t49;
        let t123 = piecewise3(t16, 0.0, -2.0 * t36 * t79 * t38 - t36 * t116 + t87 * t116);
        let t124 = t114 + t123;
        let tv2rho21 = -t25 * t26 * t124 - t103 - t105 - t59 - t61 - t66 - t68;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t129 = t45 * t45;
        let t130 = t129 * t74;
        let t133 = 2.0 * t29 + 2.0 * t79;
        let t138 = piecewise3(t11, 0.0, t27 * t133 * t32 - t27 * t130 + t71 * t130);
        let t139 = t49 * t49;
        let t140 = t139 * t90;
        let t142 = -t133;
        let t147 = piecewise3(t16, 0.0, t36 * t142 * t38 - t36 * t140 + t87 * t140);
        let t148 = t138 + t147;
        let tv2rho22 = -t25 * t26 * t148 - 2.0 * t103 - 2.0 * t105 - t59 - t66;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t151 = param_b * t29;
        let t153 = t56 * t151 * t19;
        let t155 = t56 * t57 * t42;
        let t158 = t4 * t6 * t99;
        let t160 = t63 * param_b;
        let t161 = t160 * t29;
        let t163 = t56 * t161 * t19;
        let t165 = t56 * t64 * t42;
        let t168 = t4 * t22 * t99;
        let t170 = t70 * t1;
        let t171 = t13 * t170;
        let t172 = t72 * t31;
        let t174 = 1.0 / t73 / t10;
        let t175 = t172 * t174;
        let t177 = t106 * t81;
        let t182 = t28 * t28;
        let t183 = 1.0 / t182;
        let t184 = t7 * t183;
        let t186 = 6.0 * t78 - 6.0 * t184;
        let t194 = piecewise3(t11, 0.0, t27 * t186 * t32 + t171 * t175 + 2.0 * t27 * t175 - 3.0 * t71 * t175 - 3.0 * t27 * t177 + 3.0 * t71 * t177);
        let t195 = t17 * t170;
        let t196 = t88 * t37;
        let t198 = 1.0 / t89 / t15;
        let t199 = t196 * t198;
        let t201 = t115 * t93;
        let t206 = -t186;
        let t214 = piecewise3(t16, 0.0, t36 * t206 * t38 + t195 * t199 + 2.0 * t36 * t199 - 3.0 * t87 * t199 - 3.0 * t36 * t201 + 3.0 * t87 * t201);
        let t215 = t194 + t214;
        let tv3rho30 = -t25 * t26 * t215 + t153 - 3.0 * t155 - 3.0 * t158 - t163 - 3.0 * t165 - 3.0 * t168;
        v3rho3[ip * 4] += tv3rho30;
        let t221 = t56 * t57 * t53;
        let t224 = 2.0 * t4 * t6 * t124;
        let t226 = t56 * t64 * t53;
        let t229 = 2.0 * t4 * t22 * t124;
        let t230 = t72 * t174;
        let t231 = t230 * t45;
        let t233 = t81 * t74;
        let t234 = t233 * t45;
        let t238 = t71 * t31;
        let t239 = t74 * t7;
        let t240 = t239 * t78;
        let t249 = t27 * t7;
        let t250 = t78 * t74;
        let t251 = t250 * t31;
        let t258 = piecewise3(t11, 0.0, -6.0 * t27 * t184 * t32 + 2.0 * t27 * t78 * t32 + t171 * t231 + 2.0 * t27 * t231 - 3.0 * t71 * t231 - t27 * t234 + t71 * t234 + 4.0 * t238 * t240 - 4.0 * t249 * t251);
        let t259 = t88 * t198;
        let t260 = t259 * t49;
        let t262 = t93 * t90;
        let t263 = t262 * t49;
        let t267 = t87 * t37;
        let t268 = t90 * t7;
        let t269 = t268 * t78;
        let t278 = t36 * t7;
        let t279 = t78 * t90;
        let t280 = t279 * t37;
        let t287 = piecewise3(t16, 0.0, 6.0 * t36 * t184 * t38 - 2.0 * t36 * t78 * t38 + t195 * t260 + 2.0 * t36 * t260 - 3.0 * t87 * t260 - t36 * t263 + t87 * t263 - 4.0 * t267 * t269 + 4.0 * t278 * t280);
        let t288 = t258 + t287;
        let tv3rho31 = -t25 * t26 * t288 + t153 - 2.0 * t155 - t158 - t163 - 2.0 * t165 - t168 - t221 - t224 - t226 - t229;
        v3rho3[ip * 4 + 1] += tv3rho31;
        let t294 = t4 * t6 * t148;
        let t296 = t4 * t22 * t148;
        let t297 = t31 * t174;
        let t298 = t297 * t129;
        let t300 = t71 * t45;
        let t305 = t106 * t133;
        let t309 = -2.0 * t78 - 6.0 * t184;
        let t313 = t27 * t45;
        let t319 = piecewise3(t11, 0.0, t27 * t309 * t32 + t171 * t298 + 4.0 * t300 * t240 - 4.0 * t313 * t240 + 2.0 * t27 * t298 - t27 * t305 - 3.0 * t71 * t298 + t71 * t305);
        let t320 = t37 * t198;
        let t321 = t320 * t139;
        let t323 = t87 * t49;
        let t328 = t115 * t142;
        let t330 = -t309;
        let t334 = t36 * t49;
        let t340 = piecewise3(t16, 0.0, t36 * t330 * t38 + t195 * t321 - 4.0 * t323 * t269 + 4.0 * t334 * t269 + 2.0 * t36 * t321 - 3.0 * t87 * t321 - t36 * t328 + t87 * t328);
        let t341 = t319 + t340;
        let tv3rho32 = -t25 * t26 * t341 + t153 - t155 - t163 - t165 - 2.0 * t221 - t224 - 2.0 * t226 - t229 - t294 - t296;
        v3rho3[ip * 4 + 2] += tv3rho32;
        let t348 = t129 * t45;
        let t349 = t348 * t174;
        let t351 = t45 * t74;
        let t352 = t351 * t133;
        let t358 = -6.0 * t78 - 6.0 * t184;
        let t366 = piecewise3(t11, 0.0, t27 * t358 * t32 + t171 * t349 + 2.0 * t27 * t349 - 3.0 * t27 * t352 - 3.0 * t71 * t349 + 3.0 * t71 * t352);
        let t367 = t139 * t49;
        let t368 = t367 * t198;
        let t370 = t49 * t90;
        let t371 = t370 * t142;
        let t376 = -t358;
        let t384 = piecewise3(t16, 0.0, t36 * t376 * t38 + t195 * t368 + 2.0 * t36 * t368 - 3.0 * t36 * t371 - 3.0 * t87 * t368 + 3.0 * t87 * t371);
        let t385 = t366 + t384;
        let tv3rho33 = -t25 * t26 * t385 + t153 - t163 - 3.0 * t221 - 3.0 * t226 - 3.0 * t294 - 3.0 * t296;
        v3rho3[ip * 4 + 3] += tv3rho33;
    }
}
