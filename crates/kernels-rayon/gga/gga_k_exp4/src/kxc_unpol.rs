//! GGA_K_EXP4 kxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_exp4.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_exp4_kxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3(rho[ip]);
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t24 = M_CBRT6;
        let t25 = M_PI * M_PI;
        let t26 = pow_1_3(t25);
        let t27 = t26 * t26;
        let t28 = 1.0 / t27;
        let t29 = t24 * t28;
        let t30 = M_CBRT2;
        let t31 = t30 * t30;
        let t32 = sigma[ip] * t31;
        let t33 = rho[ip] * rho[ip];
        let t35 = 1.0 / t22 / t33;
        let t39 = rmath::exp(-8.325416666666667 * t29 * t32 * t35);
        let t41 = t24 * t24;
        let t43 = 1.0 / t26 / t25;
        let t44 = t41 * t43;
        let t45 = sigma[ip] * sigma[ip];
        let t47 = t33 * t33;
        let t48 = t47 * rho[ip];
        let t50 = 1.0 / t21 / t48;
        let t54 = rmath::exp(-0.015095833333333333 * t44 * t45 * t30 * t50);
        let t56 = 2.0788 - 0.8524 * t39 - 1.2264 * t54;
        let t60 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t56);
        let tzk0 = 2.0 * t60;
        zk[ip] += tzk0;
        let t62 = t20 / t21;
        let t66 = t29 * sigma[ip];
        let t67 = t33 * rho[ip];
        let t71 = t31 / t22 / t67 * t39;
        let t74 = t44 * t45;
        let t75 = t47 * t33;
        let t77 = 1.0 / t21 / t75;
        let t78 = t30 * t77;
        let t79 = t78 * t54;
        let t82 = -18.92422711111111 * t66 * t71 - 0.09873882666666667 * t74 * t79;
        let t87 = piecewise3(t2, 0.0, t7 * t62 * t56 / 10.0 + 3.0 / 20.0 * t7 * t23 * t82);
        let tvrho0 = 2.0 * rho[ip] * t87 + 2.0 * t60;
        vrho[ip] += tvrho0;
        let t94 = t44 * sigma[ip];
        let t95 = t30 * t50;
        let t96 = t95 * t54;
        let t99 = 7.096585166666666 * t29 * t31 * t35 * t39 + 0.03702706 * t94 * t96;
        let t103 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t99);
        let tvsigma0 = 2.0 * rho[ip] * t103;
        vsigma[ip] += tvsigma0;
        let t108 = t20 / t21 / rho[ip];
        let t118 = t31 / t22 / t47 * t39;
        let t121 = t47 * t67;
        let t123 = 1.0 / t21 / t121;
        let t124 = t30 * t123;
        let t125 = t124 * t39;
        let t128 = t124 * t54;
        let t131 = t25 * t25;
        let t134 = t24 / t27 / t131;
        let t135 = t45 * t45;
        let t136 = t134 * t135;
        let t137 = t47 * t47;
        let t138 = t137 * t47;
        let t140 = 1.0 / t22 / t138;
        let t142 = t31 * t140 * t54;
        let t145 = 69.38883274074074 * t66 * t118 - 840.277737571358 * t74 * t125 + 0.6253459022222222 * t74 * t128 - 0.047697435868444445 * t136 * t142;
        let t150 = piecewise3(t2, 0.0, -t7 * t108 * t56 / 30.0 + t7 * t62 * t82 / 5.0 + 3.0 / 20.0 * t7 * t23 * t145);
        let tv2rho20 = 2.0 * rho[ip] * t150 + 4.0 * t87;
        v2rho2[ip] += tv2rho20;
        let t158 = t44 * t30;
        let t165 = t45 * sigma[ip];
        let t166 = t134 * t165;
        let t167 = t137 * t67;
        let t169 = 1.0 / t22 / t167;
        let t174 = -18.92422711111111 * t29 * t71 + 315.10415158925923 * t158 * t77 * sigma[ip] * t39 - 0.19747765333333334 * t94 * t79 + 0.017886538450666668 * t166 * t31 * t169 * t54;
        let t179 = piecewise3(t2, 0.0, t7 * t62 * t99 / 10.0 + 3.0 / 20.0 * t7 * t23 * t174);
        let tv2rhosigma0 = 2.0 * rho[ip] * t179 + 2.0 * t103;
        v2rhosigma[ip] += tv2rhosigma0;
        let t188 = t137 * t33;
        let t190 = 1.0 / t22 / t188;
        let t192 = t31 * t190 * t54;
        let t195 = -118.16405684597223 * t44 * t95 * t39 + 0.03702706 * t44 * t96 - 0.006707451919 * t134 * t45 * t192;
        let t199 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t195);
        let tv2sigma20 = 2.0 * rho[ip] * t199;
        v2sigma2[ip] += tv2sigma20;
        let t204 = t20 / t21 / t33;
        let t217 = t31 / t22 / t48 * t39;
        let t221 = 1.0 / t21 / t137;
        let t222 = t30 * t221;
        let t226 = 1.0 / t167;
        let t230 = t222 * t54;
        let t235 = 1.0 / t22 / t137 / t48;
        let t237 = t31 * t235 * t54;
        let t240 = t135 * t45;
        let t241 = t137 * t137;
        let t243 = 1.0 / t241 / t67;
        let t247 = -323.81455279012346 * t66 * t217 + 9243.055113284938 * t74 * t222 * t39 - 2298.155034770158 * t165 * t226 * t39 - 4.58586994962963 * t74 * t230 + 0.9062512815004444 * t136 * t237 - 4.856608744702908e-06 * t240 * t243 * t54;
        let t252 = piecewise3(t2, 0.0, 2.0 / 45.0 * t7 * t204 * t56 - t7 * t108 * t82 / 10.0 + 3.0 / 10.0 * t7 * t62 * t145 + 3.0 / 20.0 * t7 * t23 * t247);
        let tv3rho30 = 2.0 * rho[ip] * t252 + 6.0 * t150;
        v3rho3[ip] += tv3rho30;
        let t268 = 1.0 / t188;
        let t276 = t135 * sigma[ip];
        let t278 = 1.0 / t241 / t33;
        let t282 = 69.38883274074074 * t29 * t118 - 2835.937364303333 * t158 * t123 * sigma[ip] * t39 + 861.8081380388094 * t268 * t45 * t39 + 1.2506918044444444 * t94 * t128 - 0.30407115366133336 * t166 * t142 + 1.8212282792635907e-06 * t276 * t278 * t54;
        let t287 = piecewise3(t2, 0.0, -t7 * t108 * t99 / 30.0 + t7 * t62 * t174 / 5.0 + 3.0 / 20.0 * t7 * t23 * t282);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t287 + 4.0 * t179;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t296 = t137 * rho[ip];
        let t297 = 1.0 / t296;
        let t303 = t134 * t31;
        let t309 = 1.0 / t241 / rho[ip];
        let t313 = 630.2083031785185 * t44 * t78 * t39 - 323.1780517645535 * t297 * sigma[ip] * t39 - 0.19747765333333334 * t44 * t79 + 0.08943269225333333 * t303 * t169 * t45 * t54 - 6.829606047238465e-07 * t135 * t309 * t54;
        let t318 = piecewise3(t2, 0.0, t7 * t62 * t195 / 10.0 + 3.0 / 20.0 * t7 * t23 * t313);
        let tv3rhosigma20 = 2.0 * rho[ip] * t318 + 2.0 * t199;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t328 = 1.0 / t241;
        let t332 = 121.19176941170757 / t137 * t39 - 0.020122355757 * t303 * t190 * sigma[ip] * t54 + 2.5611022677144246e-07 * t165 * t328 * t54;
        let t336 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t332);
        let tv3sigma30 = 2.0 * rho[ip] * t336;
        v3sigma3[ip] += tv3sigma30;
    }
}
