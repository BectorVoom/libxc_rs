//! GGA_X_Q1D vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 90 shared lines across all orders.
//! Delta: 97 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_q1d_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        // --- shared preamble (90 lines) ---
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = M_PI * M_PI;
        let t30 = pow_1_3(t29);
        let t31 = t30 * t30;
        let t32 = 1.0 / t31;
        let t33 = t28 * t32;
        let t34 = rho0 * rho0;
        let t35 = pow_1_3(rho0);
        let t36 = t35 * t35;
        let t38 = 1.0 / t36 / t34;
        let t40 = t33 * sigma0 * t38;
        let t42 = 0.804e0 + 5.0 / 972.0 * t40;
        let t44 = 0.646416e0 / t42;
        let t46 = t28 * t28;
        let t48 = 1.0 / t30 / t29;
        let t49 = t46 * t48;
        let t50 = sigma0 * sigma0;
        let t51 = t34 * t34;
        let t52 = t51 * rho0;
        let t54 = 1.0 / t35 / t52;
        let t57 = t49 * t50 * t54 / 576.0;
        let t58 = t40 / 24.0 + t57;
        let t59 = t29 * t29;
        let t60 = 1.0 / t59;
        let t61 = t50 * sigma0;
        let t62 = t60 * t61;
        let t63 = t51 * t51;
        let t64 = 1.0 / t63;
        let t67 = 1.0 + t57 + t62 * t64 / 2304.0;
        let t68 = 1.0 / t67;
        let t69 = t58 * t68;
        let t71 = (0.1804e1 - t44) * t28;
        let t72 = t32 * sigma0;
        let t76 = -t71 * t72 * t38 / 24.0 + 0.6525e-1;
        let t78 = 0.1804e1 - t44 + t69 * t76;
        let t82 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t78);
        let t83 = rho1 <= dens_threshold;
        let t84 = -t16;
        let t86 = piecewise5(t14, t11, t10, t15, t84 * t7);
        let t87 = 1.0 + t86;
        let t88 = t87 <= zeta_threshold;
        let t89 = pow_1_3(t87);
        let t91 = piecewise3(t88, t22, t89 * t87);
        let t92 = t91 * t26;
        let t93 = rho1 * rho1;
        let t94 = pow_1_3(rho1);
        let t95 = t94 * t94;
        let t97 = 1.0 / t95 / t93;
        let t99 = t33 * sigma2 * t97;
        let t101 = 0.804e0 + 5.0 / 972.0 * t99;
        let t103 = 0.646416e0 / t101;
        let t105 = sigma2 * sigma2;
        let t106 = t93 * t93;
        let t107 = t106 * rho1;
        let t109 = 1.0 / t94 / t107;
        let t112 = t49 * t105 * t109 / 576.0;
        let t113 = t99 / 24.0 + t112;
        let t114 = t105 * sigma2;
        let t115 = t60 * t114;
        let t116 = t106 * t106;
        let t117 = 1.0 / t116;
        let t120 = 1.0 + t112 + t115 * t117 / 2304.0;
        let t121 = 1.0 / t120;
        let t122 = t113 * t121;
        let t124 = (0.1804e1 - t103) * t28;
        let t125 = t32 * sigma2;
        let t129 = -t124 * t125 * t97 / 24.0 + 0.6525e-1;
        let t131 = 0.1804e1 - t103 + t122 * t129;
        let t135 = piecewise3(t83, 0.0, -3.0 / 8.0 * t5 * t92 * t131);
        let tzk0 = t82 + t135;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (97 lines) ---
        let t136 = t6 * t6;
        let t137 = 1.0 / t136;
        let t138 = t16 * t137;
        let t140 = piecewise5(t10, 0.0, t14, 0.0, t7 - t138);
        let t143 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t140);
        let t144 = t143 * t26;
        let t148 = t26 * t26;
        let t149 = 1.0 / t148;
        let t150 = t25 * t149;
        let t153 = t5 * t150 * t78 / 8.0;
        let t154 = t42 * t42;
        let t155 = 1.0 / t154;
        let t156 = t155 * t28;
        let t157 = t34 * rho0;
        let t159 = 1.0 / t36 / t157;
        let t160 = t72 * t159;
        let t166 = t51 * t34;
        let t168 = 1.0 / t35 / t166;
        let t171 = t49 * t50 * t168 / 108.0;
        let t172 = -t33 * sigma0 * t159 / 9.0 - t171;
        let t173 = t172 * t68;
        let t175 = t67 * t67;
        let t176 = 1.0 / t175;
        let t177 = t58 * t176;
        let t178 = t63 * rho0;
        let t179 = 1.0 / t178;
        let t182 = -t171 - t62 * t179 / 288.0;
        let t183 = t76 * t182;
        let t185 = t155 * t46;
        let t186 = t48 * t50;
        let t192 = 0.36946502057613168724e-3 * t185 * t186 * t168 + t71 * t160 / 9.0;
        let t194 = -0.88671604938271604938e-2 * t156 * t160 + t173 * t76 - t177 * t183 + t69 * t192;
        let t199 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t144 * t78 - t153 - 3.0 / 8.0 * t5 * t27 * t194);
        let t200 = t84 * t137;
        let t202 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t200);
        let t205 = piecewise3(t88, 0.0, 4.0 / 3.0 * t89 * t202);
        let t206 = t205 * t26;
        let t210 = t91 * t149;
        let t213 = t5 * t210 * t131 / 8.0;
        let t215 = piecewise3(t83, 0.0, -3.0 / 8.0 * t5 * t206 * t131 - t213);
        let tvrho0 = t82 + t135 + t6 * (t199 + t215);
        vrho[ip * 2] += tvrho0;
        let t219 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t138);
        let t222 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t219);
        let t223 = t222 * t26;
        let t228 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t223 * t78 - t153);
        let t230 = piecewise5(t14, 0.0, t10, 0.0, t7 - t200);
        let t233 = piecewise3(t88, 0.0, 4.0 / 3.0 * t89 * t230);
        let t234 = t233 * t26;
        let t238 = t101 * t101;
        let t239 = 1.0 / t238;
        let t240 = t239 * t28;
        let t241 = t93 * rho1;
        let t243 = 1.0 / t95 / t241;
        let t244 = t125 * t243;
        let t250 = t106 * t93;
        let t252 = 1.0 / t94 / t250;
        let t255 = t49 * t105 * t252 / 108.0;
        let t256 = -t33 * sigma2 * t243 / 9.0 - t255;
        let t257 = t256 * t121;
        let t259 = t120 * t120;
        let t260 = 1.0 / t259;
        let t261 = t113 * t260;
        let t262 = t116 * rho1;
        let t263 = 1.0 / t262;
        let t266 = -t255 - t115 * t263 / 288.0;
        let t267 = t129 * t266;
        let t269 = t239 * t46;
        let t270 = t48 * t105;
        let t276 = 0.36946502057613168724e-3 * t269 * t270 * t252 + t124 * t244 / 9.0;
        let t278 = -0.88671604938271604938e-2 * t240 * t244 + t257 * t129 - t261 * t267 + t122 * t276;
        let t283 = piecewise3(t83, 0.0, -3.0 / 8.0 * t5 * t234 * t131 - t213 - 3.0 / 8.0 * t5 * t92 * t278);
        let tvrho1 = t82 + t135 + t6 * (t228 + t283);
        vrho[ip * 2 + 1] += tvrho1;
        let t286 = t32 * t38;
        let t293 = t49 * sigma0 * t54 / 288.0;
        let t294 = t33 * t38 / 24.0 + t293;
        let t295 = t294 * t68;
        let t297 = t60 * t50;
        let t300 = t293 + t297 * t64 / 768.0;
        let t301 = t76 * t300;
        let t303 = t48 * t54;
        let t309 = -0.13854938271604938272e-3 * t185 * t303 * sigma0 - t71 * t286 / 24.0;
        let t311 = 0.33251851851851851852e-2 * t156 * t286 + t295 * t76 - t177 * t301 + t69 * t309;
        let t315 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t311);
        let tvsigma0 = t6 * t315;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t316 = t32 * t97;
        let t323 = t49 * sigma2 * t109 / 288.0;
        let t324 = t33 * t97 / 24.0 + t323;
        let t325 = t324 * t121;
        let t327 = t60 * t105;
        let t330 = t323 + t327 * t117 / 768.0;
        let t331 = t129 * t330;
        let t333 = t48 * t109;
        let t339 = -0.13854938271604938272e-3 * t269 * t333 * sigma2 - t124 * t316 / 24.0;
        let t341 = 0.33251851851851851852e-2 * t240 * t316 + t325 * t129 - t261 * t331 + t122 * t339;
        let t345 = piecewise3(t83, 0.0, -3.0 / 8.0 * t5 * t92 * t341);
        let tvsigma2 = t6 * t345;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
