//! MGGA_X_MVS vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 91 shared lines across all orders.
//! Delta: 114 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_mvs_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_b: f64,
    param_c1: f64,
    param_e1: f64,
    param_k0: f64,
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
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        // --- shared preamble (91 lines) ---
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
        let t29 = pow_1_3(rho0);
        let t30 = t29 * t29;
        let t32 = 1.0 / t30 / rho0;
        let t34 = rho0 * rho0;
        let t36 = 1.0 / t30 / t34;
        let t39 = tau0 * t32 - sigma0 * t36 / 8.0;
        let t40 = M_CBRT6;
        let t42 = M_PI * M_PI;
        let t43 = pow_1_3(t42);
        let t44 = t43 * t43;
        let t45 = 1.0 / t44;
        let t49 = param_k0 * (1.0 - 5.0 / 9.0 * t39 * t40 * t45);
        let t50 = t39 * t39;
        let t52 = t40 * t40;
        let t54 = 1.0 / t43 / t42;
        let t55 = t52 * t54;
        let t58 = 1.0 + 25.0 / 81.0 * param_e1 * t50 * t55;
        let t59 = t58 * t58;
        let t60 = t50 * t50;
        let t62 = t42 * t42;
        let t64 = 1.0 / t44 / t62;
        let t65 = t40 * t64;
        let t68 = t59 + 1250.0 / 2187.0 * param_c1 * t60 * t65;
        let t69 = pow_1_4(t68);
        let t70 = 1.0 / t69;
        let t72 = t49 * t70 + 1.0;
        let t74 = param_b * t52;
        let t75 = sigma0 * sigma0;
        let t76 = t54 * t75;
        let t77 = t34 * t34;
        let t78 = t77 * rho0;
        let t80 = 1.0 / t29 / t78;
        let t84 = 1.0 + t74 * t76 * t80 / 576.0;
        let t85 = f64::powf(t84, 1.0 / 8.0);
        let t86 = 1.0 / t85;
        let t87 = t28 * t72 * t86;
        let t90 = piecewise3(t2, 0.0, -3.0 / 8.0 * t27 * t87);
        let t91 = rho1 <= dens_threshold;
        let t92 = -t17;
        let t94 = piecewise5(t15, t12, t11, t16, t92 * t8);
        let t95 = 1.0 + t94;
        let t96 = t95 <= zeta_threshold;
        let t97 = pow_1_3(t95);
        let t99 = piecewise3(t96, t23, t97 * t95);
        let t100 = t6 * t99;
        let t101 = pow_1_3(rho1);
        let t102 = t101 * t101;
        let t104 = 1.0 / t102 / rho1;
        let t106 = rho1 * rho1;
        let t108 = 1.0 / t102 / t106;
        let t111 = tau1 * t104 - sigma2 * t108 / 8.0;
        let t116 = param_k0 * (1.0 - 5.0 / 9.0 * t111 * t40 * t45);
        let t117 = t111 * t111;
        let t121 = 1.0 + 25.0 / 81.0 * param_e1 * t117 * t55;
        let t122 = t121 * t121;
        let t123 = t117 * t117;
        let t127 = t122 + 1250.0 / 2187.0 * param_c1 * t123 * t65;
        let t128 = pow_1_4(t127);
        let t129 = 1.0 / t128;
        let t131 = t116 * t129 + 1.0;
        let t133 = sigma2 * sigma2;
        let t134 = t54 * t133;
        let t135 = t106 * t106;
        let t136 = t135 * rho1;
        let t138 = 1.0 / t101 / t136;
        let t142 = 1.0 + t74 * t134 * t138 / 576.0;
        let t143 = f64::powf(t142, 1.0 / 8.0);
        let t144 = 1.0 / t143;
        let t145 = t28 * t131 * t144;
        let t148 = piecewise3(t91, 0.0, -3.0 / 8.0 * t100 * t145);
        let tzk0 = t90 + t148;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (114 lines) ---
        let t149 = t7 * t7;
        let t150 = 1.0 / t149;
        let t151 = t17 * t150;
        let t153 = piecewise5(t11, 0.0, t15, 0.0, t8 - t151);
        let t156 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t153);
        let t157 = t6 * t156;
        let t160 = t28 * t28;
        let t161 = 1.0 / t160;
        let t163 = t161 * t72 * t86;
        let t165 = t27 * t163 / 8.0;
        let t168 = t34 * rho0;
        let t170 = 1.0 / t30 / t168;
        let t173 = -5.0 / 3.0 * tau0 * t36 + sigma0 * t170 / 3.0;
        let t174 = param_k0 * t173;
        let t175 = t40 * t45;
        let t176 = t175 * t70;
        let t180 = 1.0 / t69 / t68;
        let t181 = t58 * param_e1;
        let t182 = t181 * t39;
        let t187 = param_c1 * t50 * t39;
        let t191 = 100.0 / 81.0 * t182 * t55 * t173 + 5000.0 / 2187.0 * t187 * t65 * t173;
        let t195 = -5.0 / 9.0 * t174 * t176 - t49 * t180 * t191 / 4.0;
        let t197 = t28 * t195 * t86;
        let t200 = t26 * t28;
        let t201 = t200 * t72;
        let t202 = t6 * t201;
        let t205 = 1.0 / t85 / t84 * param_b;
        let t206 = t205 * t52;
        let t207 = t77 * t34;
        let t209 = 1.0 / t29 / t207;
        let t211 = t206 * t76 * t209;
        let t215 = piecewise3(t2, 0.0, -3.0 / 8.0 * t157 * t87 - t165 - 3.0 / 8.0 * t27 * t197 - t202 * t211 / 2304.0);
        let t216 = t92 * t150;
        let t218 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t216);
        let t221 = piecewise3(t96, 0.0, 4.0 / 3.0 * t97 * t218);
        let t222 = t6 * t221;
        let t226 = t161 * t131 * t144;
        let t228 = t100 * t226 / 8.0;
        let t230 = piecewise3(t91, 0.0, -3.0 / 8.0 * t222 * t145 - t228);
        let tvrho0 = t90 + t148 + t7 * (t215 + t230);
        vrho[ip * 2] += tvrho0;
        let t234 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t151);
        let t237 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t234);
        let t238 = t6 * t237;
        let t242 = piecewise3(t2, 0.0, -3.0 / 8.0 * t238 * t87 - t165);
        let t244 = piecewise5(t15, 0.0, t11, 0.0, t8 - t216);
        let t247 = piecewise3(t96, 0.0, 4.0 / 3.0 * t97 * t244);
        let t248 = t6 * t247;
        let t253 = t106 * rho1;
        let t255 = 1.0 / t102 / t253;
        let t258 = -5.0 / 3.0 * tau1 * t108 + sigma2 * t255 / 3.0;
        let t259 = param_k0 * t258;
        let t260 = t175 * t129;
        let t264 = 1.0 / t128 / t127;
        let t265 = t121 * param_e1;
        let t266 = t265 * t111;
        let t271 = param_c1 * t117 * t111;
        let t275 = 100.0 / 81.0 * t266 * t55 * t258 + 5000.0 / 2187.0 * t271 * t65 * t258;
        let t279 = -5.0 / 9.0 * t259 * t260 - t116 * t264 * t275 / 4.0;
        let t281 = t28 * t279 * t144;
        let t284 = t99 * t28;
        let t285 = t284 * t131;
        let t286 = t6 * t285;
        let t289 = 1.0 / t143 / t142 * param_b;
        let t290 = t289 * t52;
        let t291 = t135 * t106;
        let t293 = 1.0 / t101 / t291;
        let t295 = t290 * t134 * t293;
        let t299 = piecewise3(t91, 0.0, -3.0 / 8.0 * t248 * t145 - t228 - 3.0 / 8.0 * t100 * t281 - t286 * t295 / 2304.0);
        let tvrho1 = t90 + t148 + t7 * (t242 + t299);
        vrho[ip * 2 + 1] += tvrho1;
        let t302 = param_k0 * t36;
        let t303 = t302 * t176;
        let t305 = t55 * t36;
        let t306 = t182 * t305;
        let t308 = t65 * t36;
        let t309 = t187 * t308;
        let t311 = -25.0 / 162.0 * t306 - 625.0 / 2187.0 * t309;
        let t315 = 5.0 / 72.0 * t303 - t49 * t180 * t311 / 4.0;
        let t317 = t28 * t315 * t86;
        let t320 = t54 * sigma0;
        let t322 = t206 * t320 * t80;
        let t326 = piecewise3(t2, 0.0, -3.0 / 8.0 * t27 * t317 + t202 * t322 / 6144.0);
        let tvsigma0 = t7 * t326;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t327 = param_k0 * t108;
        let t328 = t327 * t260;
        let t330 = t55 * t108;
        let t331 = t266 * t330;
        let t333 = t65 * t108;
        let t334 = t271 * t333;
        let t336 = -25.0 / 162.0 * t331 - 625.0 / 2187.0 * t334;
        let t340 = 5.0 / 72.0 * t328 - t116 * t264 * t336 / 4.0;
        let t342 = t28 * t340 * t144;
        let t345 = t54 * sigma2;
        let t347 = t290 * t345 * t138;
        let t351 = piecewise3(t91, 0.0, -3.0 / 8.0 * t100 * t342 + t286 * t347 / 6144.0);
        let tvsigma2 = t7 * t351;
        vsigma[ip * 3 + 2] += tvsigma2;
        let tvlapl0 = 0.0;
        vlapl[ip * 2] += tvlapl0;
        let tvlapl1 = 0.0;
        vlapl[ip * 2 + 1] += tvlapl1;
        let t352 = param_k0 * t32;
        let t355 = t55 * t32;
        let t358 = t65 * t32;
        let t361 = 100.0 / 81.0 * t182 * t355 + 5000.0 / 2187.0 * t187 * t358;
        let t365 = -5.0 / 9.0 * t352 * t176 - t49 * t180 * t361 / 4.0;
        let t367 = t28 * t365 * t86;
        let t370 = piecewise3(t2, 0.0, -3.0 / 8.0 * t27 * t367);
        let tvtau0 = t7 * t370;
        vtau[ip * 2] += tvtau0;
        let t371 = param_k0 * t104;
        let t374 = t55 * t104;
        let t377 = t65 * t104;
        let t380 = 100.0 / 81.0 * t266 * t374 + 5000.0 / 2187.0 * t271 * t377;
        let t384 = -5.0 / 9.0 * t371 * t260 - t116 * t264 * t380 / 4.0;
        let t386 = t28 * t384 * t144;
        let t389 = piecewise3(t91, 0.0, -3.0 / 8.0 * t100 * t386);
        let tvtau1 = t7 * t389;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
