//! MGGA_K_CSK fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_csk.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_k_csk_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
    param_csk_a: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = t4 * t4;
        let t6 = M_CBRTPI;
        let t8 = t5 * t6 * M_PI;
        let t9 = 1.0 <= zeta_threshold;
        let t10 = zeta_threshold - 1.0;
        let t12 = piecewise5(t9, t10, t9, -t10, 0.0);
        let t13 = 1.0 + t12;
        let t15 = pow_1_3(zeta_threshold);
        let t16 = t15 * t15;
        let t18 = pow_1_3(t13);
        let t19 = t18 * t18;
        let t21 = piecewise3(t13 <= zeta_threshold, t16 * zeta_threshold, t19 * t13);
        let t22 = pow_1_3(rho[ip]);
        let t23 = t22 * t22;
        let t24 = t21 * t23;
        let t25 = M_CBRT6;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3(t26);
        let t28 = t27 * t27;
        let t30 = t25 / t28;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t23 / t34;
        let t38 = t30 * t33 * t36;
        let t40 = lapl[ip] * t32;
        let t42 = 1.0 / t23 / rho[ip];
        let t47 = 5.0 / 54.0 * t30 * t40 * t42 - 5.0 / 81.0 * t38;
        let t49 = rmath::ln(1.0 - f64::EPSILON);
        let t50 = 1.0 / param_csk_a;
        let t51 = rmath::pow(-t49, -t50);
        let t52 = t47 < -t51;
        let t53 = rmath::ln(f64::EPSILON);
        let t54 = rmath::pow(-t53, -t50);
        let t55 = -t54 < t47;
        let t56 = piecewise3(t55, -t54, t47);
        let t57 = -t51 < t56;
        let t58 = piecewise3(t57, t56, -t51);
        let t59 = rmath::abs(t58);
        let t60 = rmath::pow(t59, param_csk_a);
        let t61 = 1.0 / t60;
        let t62 = rmath::exp(-t61);
        let t63 = 1.0 - t62;
        let t64 = rmath::pow(t63, t50);
        let t65 = piecewise5(t52, 0.0, t55, 1.0, t64);
        let t67 = 1.0 + 5.0 / 72.0 * t38 + t47 * t65;
        let t71 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t67);
        let tzk0 = 2.0 * t71;
        zk[ip] += tzk0;
        let t73 = t21 / t22;
        let t77 = t34 * rho[ip];
        let t79 = 1.0 / t23 / t77;
        let t81 = t30 * t33 * t79;
        let t87 = -25.0 / 162.0 * t30 * t40 * t36 + 40.0 / 243.0 * t81;
        let t89 = t64 * t61;
        let t90 = piecewise3(t55, 0.0, t87);
        let t91 = piecewise3(t57, t90, 0.0);
        let t93 = rmath::abs(t58) / t58;
        let t94 = 1.0 / t59;
        let t96 = 1.0 / t63;
        let t97 = t62 * t96;
        let t98 = t93 * t94 * t97;
        let t100 = piecewise5(t52, 0.0, t55, 0.0, -t89 * t91 * t98);
        let t102 = -5.0 / 27.0 * t81 + t87 * t65 + t47 * t100;
        let t107 = piecewise3(t3, 0.0, t8 * t73 * t67 / 10.0 + 3.0 / 20.0 * t8 * t24 * t102);
        let tvrho0 = 2.0 * rho[ip] * t107 + 2.0 * t71;
        vrho[ip] += tvrho0;
        let t110 = t32 * t36;
        let t111 = t30 * t110;
        let t114 = t30 * t110 * t65;
        let t117 = piecewise3(t55, 0.0, -5.0 / 81.0 * t111);
        let t118 = piecewise3(t57, t117, 0.0);
        let t121 = piecewise5(t52, 0.0, t55, 0.0, -t89 * t118 * t98);
        let t123 = 5.0 / 72.0 * t111 - 5.0 / 81.0 * t114 + t47 * t121;
        let t127 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t123);
        let tvsigma0 = 2.0 * rho[ip] * t127;
        vsigma[ip] += tvsigma0;
        let t129 = t32 * t42;
        let t135 = piecewise3(t55, 0.0, 5.0 / 54.0 * t30 * t129);
        let t136 = piecewise3(t57, t135, 0.0);
        let t139 = piecewise5(t52, 0.0, t55, 0.0, -t89 * t136 * t98);
        let t141 = 5.0 / 54.0 * t30 * t129 * t65 + t47 * t139;
        let t145 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t141);
        let tvlapl0 = 2.0 * rho[ip] * t145;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
        let t150 = t21 / t22 / rho[ip];
        let t157 = t34 * t34;
        let t159 = 1.0 / t23 / t157;
        let t161 = t30 * t33 * t159;
        let t167 = 100.0 / 243.0 * t30 * t40 * t79 - 440.0 / 729.0 * t161;
        let t171 = t60 * t60;
        let t173 = t64 / t171;
        let t174 = t91 * t91;
        let t176 = t93 * t93;
        let t177 = t59 * t59;
        let t178 = 1.0 / t177;
        let t179 = t176 * t178;
        let t180 = t62 * t62;
        let t181 = t63 * t63;
        let t182 = 1.0 / t181;
        let t183 = t180 * t182;
        let t184 = t179 * t183;
        let t186 = t174 * t176;
        let t188 = t178 * t62;
        let t189 = t96 * param_csk_a;
        let t190 = t188 * t189;
        let t192 = piecewise3(t55, 0.0, t167);
        let t193 = piecewise3(t57, t192, 0.0);
        let t196 = t89 * t174;
        let t197 = 0.0;
        let t199 = t197 * t94 * t97;
        let t200 = t196 * t199;
        let t201 = t179 * t97;
        let t203 = t173 * t186;
        let t205 = t178 * t180;
        let t206 = t182 * param_csk_a;
        let t207 = t205 * t206;
        let t210 = piecewise5(t52, 0.0, t55, 0.0, t173 * t174 * t184 + t89 * t186 * t190 - t89 * t193 * t98 - t203 * t190 + t196 * t201 - t203 * t207 - t200);
        let t212 = 55.0 / 81.0 * t161 + t167 * t65 + 2.0 * t87 * t100 + t47 * t210;
        let t217 = piecewise3(t3, 0.0, -t8 * t150 * t67 / 30.0 + t8 * t73 * t102 / 5.0 + 3.0 / 20.0 * t8 * t24 * t212);
        let tv2rho20 = 2.0 * rho[ip] * t217 + 4.0 * t107;
        v2rho2[ip] += tv2rho20;
        let t223 = t32 * t79;
        let t224 = t30 * t223;
        let t227 = t30 * t223 * t65;
        let t230 = t30 * t110 * t100;
        let t233 = t91 * t176;
        let t234 = t173 * t233;
        let t235 = t182 * t118;
        let t236 = t205 * t235;
        let t238 = t118 * t176;
        let t239 = t89 * t238;
        let t240 = t189 * t91;
        let t241 = t188 * t240;
        let t244 = piecewise3(t55, 0.0, 40.0 / 243.0 * t224);
        let t245 = piecewise3(t57, t244, 0.0);
        let t248 = t118 * t91;
        let t250 = t89 * t248 * t199;
        let t251 = t96 * t91;
        let t252 = t188 * t251;
        let t254 = t173 * t238;
        let t256 = t206 * t91;
        let t257 = t205 * t256;
        let t260 = piecewise5(t52, 0.0, t55, 0.0, -t89 * t245 * t98 + t234 * t236 + t239 * t241 + t239 * t252 - t254 * t241 - t254 * t257 - t250);
        let t262 = -5.0 / 27.0 * t224 + 40.0 / 243.0 * t227 - 5.0 / 81.0 * t230 + t87 * t121 + t47 * t260;
        let t267 = piecewise3(t3, 0.0, t8 * t73 * t123 / 10.0 + 3.0 / 20.0 * t8 * t24 * t262);
        let tv2rhosigma0 = 2.0 * rho[ip] * t267 + 2.0 * t127;
        v2rhosigma[ip] += tv2rhosigma0;
        let t278 = t182 * t136;
        let t279 = t205 * t278;
        let t281 = t136 * t176;
        let t282 = t89 * t281;
        let t285 = piecewise3(t55, 0.0, -25.0 / 162.0 * t111);
        let t286 = piecewise3(t57, t285, 0.0);
        let t289 = t136 * t91;
        let t291 = t89 * t289 * t199;
        let t293 = t173 * t281;
        let t297 = piecewise5(t52, 0.0, t55, 0.0, -t89 * t286 * t98 + t234 * t279 + t282 * t241 - t293 * t241 + t282 * t252 - t293 * t257 - t291);
        let t299 = -25.0 / 162.0 * t114 + 5.0 / 54.0 * t30 * t129 * t100 + t87 * t139 + t47 * t297;
        let t304 = piecewise3(t3, 0.0, t8 * t73 * t141 / 10.0 + 3.0 / 20.0 * t8 * t24 * t299);
        let tv2rholapl0 = 2.0 * rho[ip] * t304 + 2.0 * t145;
        v2rholapl[ip] += tv2rholapl0;
        let tv2rhotau0 = 0.0;
        v2rhotau[ip] += tv2rhotau0;
        let t308 = t30 * t110 * t121;
        let t310 = t118 * t118;
        let t313 = t310 * t176;
        let t316 = piecewise3(t55, 0.0, 0.0);
        let t317 = piecewise3(t57, t316, 0.0);
        let t319 = t89 * t317 * t98;
        let t320 = t89 * t310;
        let t321 = t320 * t199;
        let t323 = t173 * t313;
        let t327 = piecewise5(t52, 0.0, t55, 0.0, t173 * t310 * t184 + t89 * t313 * t190 - t323 * t190 + t320 * t201 - t323 * t207 - t319 - t321);
        let t329 = -10.0 / 81.0 * t308 + t47 * t327;
        let t333 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t329);
        let tv2sigma20 = 2.0 * rho[ip] * t333;
        v2sigma2[ip] += tv2sigma20;
        let t339 = t30 * t110 * t139;
        let t343 = t188 * t189 * t118;
        let t345 = t136 * t118;
        let t346 = t89 * t345;
        let t347 = t346 * t199;
        let t348 = t96 * t118;
        let t349 = t188 * t348;
        let t352 = t206 * t118;
        let t353 = t205 * t352;
        let t356 = piecewise5(t52, 0.0, t55, 0.0, t254 * t279 + t282 * t343 + t282 * t349 - t293 * t343 - t293 * t353 - t319 - t347);
        let t358 = 5.0 / 54.0 * t30 * t129 * t121 - 5.0 / 81.0 * t339 + t47 * t356;
        let t362 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t358);
        let tv2sigmalapl0 = 2.0 * rho[ip] * t362;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let tv2sigmatau0 = 0.0;
        v2sigmatau[ip] += tv2sigmatau0;
        let t367 = t136 * t136;
        let t370 = t367 * t176;
        let t373 = t89 * t367;
        let t374 = t373 * t199;
        let t376 = t173 * t370;
        let t380 = piecewise5(t52, 0.0, t55, 0.0, t173 * t367 * t184 + t89 * t370 * t190 - t376 * t190 + t373 * t201 - t376 * t207 - t319 - t374);
        let t382 = 5.0 / 27.0 * t30 * t129 * t139 + t47 * t380;
        let t386 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t382);
        let tv2lapl20 = 2.0 * rho[ip] * t386;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let tv2tau20 = 0.0;
        v2tau2[ip] += tv2tau20;
    }
}
