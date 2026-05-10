//! GGA_X_LG93 vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 97 shared lines across all orders.
//! Delta: 99 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_lg93_vxc_pol(
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
        // --- shared preamble (97 lines) ---
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
        let t26 = t5 * t25;
        let t27 = pow_1_3(t6);
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
        let t42 = t28 * t28;
        let t44 = 1.0 / t30 / t29;
        let t45 = t42 * t44;
        let t46 = sigma0 * sigma0;
        let t47 = t34 * t34;
        let t48 = t47 * rho0;
        let t50 = 1.0 / t35 / t48;
        let t54 = t46 * sigma0;
        let t55 = t47 * t47;
        let t56 = 1.0 / t55;
        let t59 = t29 * t29;
        let t62 = t28 / t31 / t59;
        let t63 = t46 * t46;
        let t64 = t55 * t34;
        let t66 = 1.0 / t36 / t64;
        let t73 = t42 / t30 / t59 / t29;
        let t74 = t63 * sigma0;
        let t75 = t55 * t48;
        let t77 = 1.0 / t35 / t75;
        let t81 = t63 * t46;
        let t82 = t55 * t55;
        let t83 = 1.0 / t82;
        let t86 = 1.0 + 0.20588079936467259283e0 * t40 + 0.51718749999999999998e-1 * t45 * t46 * t50 + 0.99883908074331051182e-4 * t54 * t56 + 0.21916594328703703703e-3 * t62 * t63 * t66 + 0.11831024546682098765e-2 * t73 * t74 * t77 + 0.11106816177675317211e-8 * t81 * t83;
        let t87 = f64::powf(t86, 0.24974e-1);
        let t88 = t27 * t87;
        let t90 = 1.0 + 0.41666666666666666666e-9 * t40;
        let t91 = 1.0 / t90;
        let t92 = t88 * t91;
        let t95 = piecewise3(t1, 0.0, -3.0 / 8.0 * t26 * t92);
        let t96 = rho1 <= dens_threshold;
        let t97 = -t16;
        let t99 = piecewise5(t14, t11, t10, t15, t97 * t7);
        let t100 = 1.0 + t99;
        let t101 = t100 <= zeta_threshold;
        let t102 = pow_1_3(t100);
        let t104 = piecewise3(t101, t22, t102 * t100);
        let t105 = t5 * t104;
        let t106 = rho1 * rho1;
        let t107 = pow_1_3(rho1);
        let t108 = t107 * t107;
        let t110 = 1.0 / t108 / t106;
        let t112 = t33 * sigma2 * t110;
        let t114 = sigma2 * sigma2;
        let t115 = t106 * t106;
        let t116 = t115 * rho1;
        let t118 = 1.0 / t107 / t116;
        let t122 = t114 * sigma2;
        let t123 = t115 * t115;
        let t124 = 1.0 / t123;
        let t127 = t114 * t114;
        let t128 = t123 * t106;
        let t130 = 1.0 / t108 / t128;
        let t134 = t127 * sigma2;
        let t135 = t123 * t116;
        let t137 = 1.0 / t107 / t135;
        let t141 = t127 * t114;
        let t142 = t123 * t123;
        let t143 = 1.0 / t142;
        let t146 = 1.0 + 0.20588079936467259283e0 * t112 + 0.51718749999999999998e-1 * t45 * t114 * t118 + 0.99883908074331051182e-4 * t122 * t124 + 0.21916594328703703703e-3 * t62 * t127 * t130 + 0.11831024546682098765e-2 * t73 * t134 * t137 + 0.11106816177675317211e-8 * t141 * t143;
        let t147 = f64::powf(t146, 0.24974e-1);
        let t148 = t27 * t147;
        let t150 = 1.0 + 0.41666666666666666666e-9 * t112;
        let t151 = 1.0 / t150;
        let t152 = t148 * t151;
        let t155 = piecewise3(t96, 0.0, -3.0 / 8.0 * t105 * t152);
        let tzk0 = t95 + t155;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (99 lines) ---
        let t156 = t6 * t6;
        let t157 = 1.0 / t156;
        let t158 = t16 * t157;
        let t160 = piecewise5(t10, 0.0, t14, 0.0, t7 - t158);
        let t163 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t160);
        let t164 = t5 * t163;
        let t167 = t27 * t27;
        let t168 = 1.0 / t167;
        let t169 = t168 * t87;
        let t170 = t169 * t91;
        let t172 = t26 * t170 / 8.0;
        let t173 = f64::powf(t86, -0.975026e0);
        let t174 = t27 * t173;
        let t175 = t34 * rho0;
        let t177 = 1.0 / t36 / t175;
        let t178 = sigma0 * t177;
        let t181 = t47 * t34;
        let t183 = 1.0 / t35 / t181;
        let t187 = t55 * rho0;
        let t188 = 1.0 / t187;
        let t191 = t55 * t175;
        let t193 = 1.0 / t36 / t191;
        let t197 = t55 * t181;
        let t199 = 1.0 / t35 / t197;
        let t203 = t82 * rho0;
        let t204 = 1.0 / t203;
        let t207 = -0.54901546497246024755e0 * t33 * t178 - 0.27583333333333333332e0 * t45 * t46 * t183 - 0.79907126459464840946e-3 * t54 * t188 - 0.23377700617283950617e-2 * t62 * t63 * t193 - 0.15774699395576131687e-1 * t73 * t74 * t199 - 0.17770905884280507538e-7 * t81 * t204;
        let t208 = t91 * t207;
        let t209 = t174 * t208;
        let t212 = t2 * t25;
        let t213 = t212 * t88;
        let t214 = t90 * t90;
        let t215 = 1.0 / t214;
        let t216 = t215 * t28;
        let t217 = t32 * sigma0;
        let t218 = t217 * t177;
        let t219 = t216 * t218;
        let t223 = piecewise3(t1, 0.0, -3.0 / 8.0 * t164 * t92 - t172 - 0.936525e-2 * t26 * t209 - 0.28449335968970653394e-9 * t213 * t219);
        let t224 = t97 * t157;
        let t226 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t224);
        let t229 = piecewise3(t101, 0.0, 4.0 / 3.0 * t102 * t226);
        let t230 = t5 * t229;
        let t233 = t168 * t147;
        let t234 = t233 * t151;
        let t236 = t105 * t234 / 8.0;
        let t238 = piecewise3(t96, 0.0, -3.0 / 8.0 * t230 * t152 - t236);
        let tvrho0 = t95 + t155 + t6 * (t223 + t238);
        vrho[ip * 2] += tvrho0;
        let t242 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t158);
        let t245 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t242);
        let t246 = t5 * t245;
        let t250 = piecewise3(t1, 0.0, -3.0 / 8.0 * t246 * t92 - t172);
        let t252 = piecewise5(t14, 0.0, t10, 0.0, t7 - t224);
        let t255 = piecewise3(t101, 0.0, 4.0 / 3.0 * t102 * t252);
        let t256 = t5 * t255;
        let t259 = f64::powf(t146, -0.975026e0);
        let t260 = t27 * t259;
        let t261 = t106 * rho1;
        let t263 = 1.0 / t108 / t261;
        let t264 = sigma2 * t263;
        let t267 = t115 * t106;
        let t269 = 1.0 / t107 / t267;
        let t273 = t123 * rho1;
        let t274 = 1.0 / t273;
        let t277 = t123 * t261;
        let t279 = 1.0 / t108 / t277;
        let t283 = t123 * t267;
        let t285 = 1.0 / t107 / t283;
        let t289 = t142 * rho1;
        let t290 = 1.0 / t289;
        let t293 = -0.54901546497246024755e0 * t33 * t264 - 0.27583333333333333332e0 * t45 * t114 * t269 - 0.79907126459464840946e-3 * t122 * t274 - 0.23377700617283950617e-2 * t62 * t127 * t279 - 0.15774699395576131687e-1 * t73 * t134 * t285 - 0.17770905884280507538e-7 * t141 * t290;
        let t294 = t151 * t293;
        let t295 = t260 * t294;
        let t298 = t2 * t104;
        let t299 = t298 * t148;
        let t300 = t150 * t150;
        let t301 = 1.0 / t300;
        let t302 = t301 * t28;
        let t303 = t32 * sigma2;
        let t304 = t303 * t263;
        let t305 = t302 * t304;
        let t309 = piecewise3(t96, 0.0, -3.0 / 8.0 * t256 * t152 - t236 - 0.936525e-2 * t105 * t295 - 0.28449335968970653394e-9 * t299 * t305);
        let tvrho1 = t95 + t155 + t6 * (t250 + t309);
        vrho[ip * 2 + 1] += tvrho1;
        let t312 = t33 * t38;
        let t327 = 0.20588079936467259283e0 * t312 + 0.1034375e0 * t45 * sigma0 * t50 + 0.29965172422299315355e-3 * t46 * t56 + 0.87666377314814814812e-3 * t62 * t54 * t66 + 0.59155122733410493825e-2 * t73 * t63 * t77 + 0.66640897066051903266e-8 * t74 * t83;
        let t328 = t91 * t327;
        let t329 = t174 * t328;
        let t332 = t32 * t38;
        let t333 = t216 * t332;
        let t337 = piecewise3(t1, 0.0, -0.936525e-2 * t26 * t329 + 0.10668500988363995023e-9 * t213 * t333);
        let tvsigma0 = t6 * t337;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t338 = t33 * t110;
        let t353 = 0.20588079936467259283e0 * t338 + 0.1034375e0 * t45 * sigma2 * t118 + 0.29965172422299315355e-3 * t114 * t124 + 0.87666377314814814812e-3 * t62 * t122 * t130 + 0.59155122733410493825e-2 * t73 * t127 * t137 + 0.66640897066051903266e-8 * t134 * t143;
        let t354 = t151 * t353;
        let t355 = t260 * t354;
        let t358 = t32 * t110;
        let t359 = t302 * t358;
        let t363 = piecewise3(t96, 0.0, -0.936525e-2 * t105 * t355 + 0.10668500988363995023e-9 * t299 * t359);
        let tvsigma2 = t6 * t363;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
