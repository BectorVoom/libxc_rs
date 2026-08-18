//! LDA_X fxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_x_fxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    param_alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t8 = rho0 * t7;
        let t10 = 2.0 * t8 <= zeta_threshold;
        let t11 = pow_1_3(zeta_threshold);
        let t12 = t11 * zeta_threshold;
        let t13 = M_CBRT2;
        let t14 = t13 * rho0;
        let t15 = pow_1_3(t8);
        let t19 = piecewise3(t10, t12, 2.0 * t14 * t7 * t15);
        let t20 = pow_1_3(t6);
        let t24 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t19 * t20);
        let t25 = param_alpha * t24;
        let t26 = rho1 <= dens_threshold;
        let t27 = rho1 * t7;
        let t29 = 2.0 * t27 <= zeta_threshold;
        let t30 = t13 * rho1;
        let t31 = pow_1_3(t27);
        let t35 = piecewise3(t29, t12, 2.0 * t30 * t7 * t31);
        let t39 = piecewise3(t26, 0.0, -3.0 / 8.0 * t5 * t35 * t20);
        let t40 = param_alpha * t39;
        let tzk0 = t25 + t40;
        zk[ip] += tzk0;
        let t41 = t13 * t7;
        let t44 = t6 * t6;
        let t45 = 1.0 / t44;
        let t48 = 2.0 * t14 * t45 * t15;
        let t49 = t15 * t15;
        let t50 = 1.0 / t49;
        let t51 = t7 * t50;
        let t53 = -rho0 * t45 + t7;
        let t58 = piecewise3(t10, 0.0, 2.0 * t41 * t15 - t48 + 2.0 / 3.0 * t14 * t51 * t53);
        let t62 = t20 * t20;
        let t63 = 1.0 / t62;
        let t66 = t5 * t19 * t63 / 8.0;
        let t68 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t58 * t20 - t66);
        let t69 = param_alpha * t68;
        let t72 = 2.0 * t30 * t45 * t31;
        let t73 = rho1 * rho1;
        let t74 = t13 * t73;
        let t75 = t44 * t6;
        let t76 = 1.0 / t75;
        let t77 = t31 * t31;
        let t78 = 1.0 / t77;
        let t79 = t76 * t78;
        let t83 = piecewise3(t29, 0.0, -t72 - 2.0 / 3.0 * t74 * t79);
        let t89 = t5 * t35 * t63 / 8.0;
        let t91 = piecewise3(t26, 0.0, -3.0 / 8.0 * t5 * t83 * t20 - t89);
        let t92 = param_alpha * t91;
        let tvrho0 = t25 + t40 + t6 * (t69 + t92);
        vrho[ip * 2] += tvrho0;
        let t95 = rho0 * rho0;
        let t96 = t13 * t95;
        let t97 = t76 * t50;
        let t101 = piecewise3(t10, 0.0, -t48 - 2.0 / 3.0 * t96 * t97);
        let t106 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t101 * t20 - t66);
        let t107 = param_alpha * t106;
        let t110 = t7 * t78;
        let t112 = -rho1 * t45 + t7;
        let t117 = piecewise3(t29, 0.0, 2.0 * t41 * t31 - t72 + 2.0 / 3.0 * t30 * t110 * t112);
        let t122 = piecewise3(t26, 0.0, -3.0 / 8.0 * t5 * t117 * t20 - t89);
        let t123 = param_alpha * t122;
        let tvrho1 = t25 + t40 + t6 * (t107 + t123);
        vrho[ip * 2 + 1] += tvrho1;
        let t128 = t13 * t45;
        let t129 = t128 * t15;
        let t131 = t50 * t53;
        let t136 = 4.0 * t14 * t76 * t15;
        let t137 = t45 * t50;
        let t139 = t14 * t137 * t53;
        let t142 = 1.0 / t49 / t8;
        let t143 = t7 * t142;
        let t144 = t53 * t53;
        let t150 = 2.0 * rho0 * t76 - 2.0 * t45;
        let t155 = piecewise3(t10, 0.0, -4.0 * t129 + 4.0 / 3.0 * t41 * t131 + t136 - 4.0 / 3.0 * t139 - 4.0 / 9.0 * t14 * t143 * t144 + 2.0 / 3.0 * t14 * t51 * t150);
        let t160 = t5 * t58 * t63;
        let t163 = 1.0 / t62 / t6;
        let t166 = t5 * t19 * t163 / 12.0;
        let t168 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t155 * t20 - t160 / 4.0 + t166);
        let t169 = param_alpha * t168;
        let t172 = 4.0 * t30 * t76 * t31;
        let t173 = t44 * t44;
        let t174 = 1.0 / t173;
        let t175 = t174 * t78;
        let t176 = t74 * t175;
        let t178 = t73 * rho1;
        let t179 = t13 * t178;
        let t181 = 1.0 / t173 / t6;
        let t183 = 1.0 / t77 / t27;
        let t184 = t181 * t183;
        let t188 = piecewise3(t29, 0.0, t172 + 8.0 / 3.0 * t176 - 4.0 / 9.0 * t179 * t184);
        let t193 = t5 * t83 * t63;
        let t197 = t5 * t35 * t163 / 12.0;
        let t199 = piecewise3(t26, 0.0, -3.0 / 8.0 * t5 * t188 * t20 - t193 / 4.0 + t197);
        let t200 = param_alpha * t199;
        let tv2rho20 = 2.0 * t69 + 2.0 * t92 + t6 * (t169 + t200);
        v2rho2[ip * 3] += tv2rho20;
        let t207 = t174 * t50;
        let t208 = t96 * t207;
        let t210 = t76 * t142;
        let t211 = t210 * t53;
        let t215 = piecewise3(t10, 0.0, -2.0 * t129 + t136 - 2.0 / 3.0 * t139 - 4.0 / 3.0 * t14 * t97 + 2.0 * t208 + 4.0 / 9.0 * t96 * t211);
        let t220 = t5 * t101 * t63;
        let t224 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t215 * t20 - t220 / 8.0 - t160 / 8.0 + t166);
        let t225 = param_alpha * t224;
        let t226 = t128 * t31;
        let t228 = t13 * t76;
        let t229 = t78 * rho1;
        let t233 = t45 * t78;
        let t235 = t30 * t233 * t112;
        let t237 = t76 * t183;
        let t241 = rho1 * t76;
        let t243 = -t45 + 2.0 * t241;
        let t248 = piecewise3(t29, 0.0, -2.0 * t226 - 2.0 / 3.0 * t228 * t229 + t172 + 2.0 / 3.0 * t176 - 2.0 / 3.0 * t235 + 4.0 / 9.0 * t74 * t237 * t112 + 2.0 / 3.0 * t30 * t110 * t243);
        let t253 = t5 * t117 * t63;
        let t257 = piecewise3(t26, 0.0, -3.0 / 8.0 * t5 * t248 * t20 - t253 / 8.0 - t193 / 8.0 + t197);
        let t258 = param_alpha * t257;
        let tv2rho21 = t69 + t92 + t107 + t123 + t6 * (t225 + t258);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t264 = t95 * rho0;
        let t265 = t13 * t264;
        let t266 = t181 * t142;
        let t270 = piecewise3(t10, 0.0, t136 + 8.0 / 3.0 * t208 - 4.0 / 9.0 * t265 * t266);
        let t276 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t270 * t20 - t220 / 4.0 + t166);
        let t277 = param_alpha * t276;
        let t279 = t78 * t112;
        let t283 = t7 * t183;
        let t284 = t112 * t112;
        let t289 = -2.0 * t45 + 2.0 * t241;
        let t294 = piecewise3(t29, 0.0, -4.0 * t226 + 4.0 / 3.0 * t41 * t279 + t172 - 4.0 / 3.0 * t235 - 4.0 / 9.0 * t30 * t283 * t284 + 2.0 / 3.0 * t30 * t110 * t289);
        let t300 = piecewise3(t26, 0.0, -3.0 / 8.0 * t5 * t294 * t20 - t253 / 4.0 + t197);
        let t301 = param_alpha * t300;
        let tv2rho22 = 2.0 * t107 + 2.0 * t123 + t6 * (t277 + t301);
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}
