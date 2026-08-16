//! MGGA_K_PGSLB lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_pgslb.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_k_pgslb_lxc_unpol(
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
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rho2lapl: &mut [f64],
    v3rho2tau: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3rhosigmalapl: &mut [f64],
    v3rhosigmatau: &mut [f64],
    v3rholapl2: &mut [f64],
    v3rholapltau: &mut [f64],
    v3rhotau2: &mut [f64],
    v3sigma3: &mut [f64],
    v3sigma2lapl: &mut [f64],
    v3sigma2tau: &mut [f64],
    v3sigmalapl2: &mut [f64],
    v3sigmalapltau: &mut [f64],
    v3sigmatau2: &mut [f64],
    v3lapl3: &mut [f64],
    v3lapl2tau: &mut [f64],
    v3lapltau2: &mut [f64],
    v3tau3: &mut [f64],
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho3lapl: &mut [f64],
    v4rho3tau: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rho2sigmalapl: &mut [f64],
    v4rho2sigmatau: &mut [f64],
    v4rho2lapl2: &mut [f64],
    v4rho2lapltau: &mut [f64],
    v4rho2tau2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4rhosigma2lapl: &mut [f64],
    v4rhosigma2tau: &mut [f64],
    v4rhosigmalapl2: &mut [f64],
    v4rhosigmalapltau: &mut [f64],
    v4rhosigmatau2: &mut [f64],
    v4rholapl3: &mut [f64],
    v4rholapl2tau: &mut [f64],
    v4rholapltau2: &mut [f64],
    v4rhotau3: &mut [f64],
    v4sigma4: &mut [f64],
    v4sigma3lapl: &mut [f64],
    v4sigma3tau: &mut [f64],
    v4sigma2lapl2: &mut [f64],
    v4sigma2lapltau: &mut [f64],
    v4sigma2tau2: &mut [f64],
    v4sigmalapl3: &mut [f64],
    v4sigmalapl2tau: &mut [f64],
    v4sigmalapltau2: &mut [f64],
    v4sigmatau3: &mut [f64],
    v4lapl4: &mut [f64],
    v4lapl3tau: &mut [f64],
    v4lapl2tau2: &mut [f64],
    v4lapltau3: &mut [f64],
    v4tau4: &mut [f64],
    param_pgslb_beta: f64,
    param_pgslb_mu: f64,
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
        let t29 = 1.0 / t28;
        let t30 = t25 * t29;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t23 / t34;
        let t37 = t33 * t36;
        let t41 = param_pgslb_mu * t25 * t29;
        let t44 = f64::exp(-t41 * t37 / 24.0);
        let t45 = t25 * t25;
        let t46 = param_pgslb_beta * t45;
        let t48 = 1.0 / t27 / t26;
        let t49 = t46 * t48;
        let t50 = lapl[ip] * lapl[ip];
        let t51 = t50 * t31;
        let t52 = t34 * rho[ip];
        let t54 = 1.0 / t22 / t52;
        let t58 = 5.0 / 72.0 * t30 * t37 + t44 + t49 * t51 * t54 / 288.0;
        let t62 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t58);
        let tzk0 = 2.0 * t62;
        zk[ip] += tzk0;
        let t64 = t21 / t22;
        let t69 = 1.0 / t23 / t52;
        let t77 = t34 * t34;
        let t83 = -5.0 / 27.0 * t30 * t33 * t69 + t41 * t33 * t69 * t44 / 9.0 - 5.0 / 432.0 * t49 * t51 / t22 / t77;
        let t88 = piecewise3(t3, 0.0, t8 * t64 * t58 / 10.0 + 3.0 / 20.0 * t8 * t24 * t83);
        let tvrho0 = 2.0 * rho[ip] * t88 + 2.0 * t62;
        vrho[ip] += tvrho0;
        let t91 = t32 * t36;
        let t97 = 5.0 / 72.0 * t30 * t91 - t41 * t91 * t44 / 24.0;
        let t101 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t97);
        let tvsigma0 = 2.0 * rho[ip] * t101;
        vsigma[ip] += tvsigma0;
        let t104 = t8 * t21 * t36;
        let t107 = t46 * t48 * lapl[ip] * t31;
        let t110 = piecewise3(t3, 0.0, t104 * t107 / 960.0);
        let tvlapl0 = 2.0 * rho[ip] * t110;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
        let t115 = t21 / t22 / rho[ip];
        let t123 = 1.0 / t23 / t77;
        let t131 = param_pgslb_mu * param_pgslb_mu;
        let t132 = t131 * t45;
        let t133 = t132 * t48;
        let t134 = sigma[ip] * sigma[ip];
        let t135 = t134 * t31;
        let t138 = 1.0 / t22 / t77 / t52;
        let t143 = t77 * rho[ip];
        let t149 = 55.0 / 81.0 * t30 * t33 * t123 - 11.0 / 27.0 * t41 * t33 * t123 * t44 + 2.0 / 81.0 * t133 * t135 * t138 * t44 + 65.0 / 1296.0 * t49 * t51 / t22 / t143;
        let t154 = piecewise3(t3, 0.0, -t8 * t115 * t58 / 30.0 + t8 * t64 * t83 / 5.0 + 3.0 / 20.0 * t8 * t24 * t149);
        let tv2rho20 = 2.0 * rho[ip] * t154 + 4.0 * t88;
        v2rho2[ip] += tv2rho20;
        let t160 = t32 * t69;
        let t166 = t77 * t34;
        let t168 = 1.0 / t22 / t166;
        let t170 = sigma[ip] * t44;
        let t174 = -5.0 / 27.0 * t30 * t160 + t41 * t160 * t44 / 9.0 - t133 * t31 * t168 * t170 / 108.0;
        let t179 = piecewise3(t3, 0.0, t8 * t64 * t97 / 10.0 + 3.0 / 20.0 * t8 * t24 * t174);
        let tv2rhosigma0 = 2.0 * rho[ip] * t179 + 2.0 * t101;
        v2rhosigma[ip] += tv2rhosigma0;
        let t183 = t8 * t21 * t69;
        let t186 = piecewise3(t3, 0.0, -t183 * t107 / 360.0);
        let tv2rholapl0 = 2.0 * rho[ip] * t186 + 2.0 * t110;
        v2rholapl[ip] += tv2rholapl0;
        let tv2rhotau0 = 0.0;
        v2rhotau[ip] += tv2rhotau0;
        let t190 = t8 * t21 * t123;
        let t191 = t48 * t31;
        let t193 = t132 * t191 * t44;
        let t196 = piecewise3(t3, 0.0, t190 * t193 / 1920.0);
        let tv2sigma20 = 2.0 * rho[ip] * t196;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let tv2sigmatau0 = 0.0;
        v2sigmatau[ip] += tv2sigmatau0;
        let t198 = t46 * t191;
        let t201 = piecewise3(t3, 0.0, t104 * t198 / 960.0);
        let tv2lapl20 = 2.0 * rho[ip] * t201;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let tv2tau20 = 0.0;
        v2tau2[ip] += tv2tau20;
        let t206 = t21 / t22 / t34;
        let t217 = 1.0 / t23 / t143;
        let t225 = t77 * t77;
        let t227 = 1.0 / t22 / t225;
        let t232 = t131 * param_pgslb_mu;
        let t233 = t26 * t26;
        let t234 = 1.0 / t233;
        let t235 = t232 * t234;
        let t236 = t134 * sigma[ip];
        let t238 = 1.0 / t225 / t52;
        let t246 = -770.0 / 243.0 * t30 * t33 * t217 + 154.0 / 81.0 * t41 * t33 * t217 * t44 - 22.0 / 81.0 * t133 * t135 * t227 * t44 + 8.0 / 243.0 * t235 * t236 * t238 * t44 - 65.0 / 243.0 * t49 * t51 * t168;
        let t251 = piecewise3(t3, 0.0, 2.0 / 45.0 * t8 * t206 * t58 - t8 * t115 * t83 / 10.0 + 3.0 / 10.0 * t8 * t64 * t149 + 3.0 / 20.0 * t8 * t24 * t246);
        let tv3rho30 = 2.0 * rho[ip] * t251 + 6.0 * t154;
        v3rho3[ip] += tv3rho30;
        let t261 = t32 * t123;
        let t272 = 1.0 / t225 / t34;
        let t277 = 55.0 / 81.0 * t30 * t261 - 11.0 / 27.0 * t41 * t261 * t44 + t133 * t31 * t138 * t170 / 12.0 - t235 * t272 * t134 * t44 / 81.0;
        let t282 = piecewise3(t3, 0.0, -t8 * t115 * t97 / 30.0 + t8 * t64 * t174 / 5.0 + 3.0 / 20.0 * t8 * t24 * t277);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t282 + 4.0 * t179;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t288 = piecewise3(t3, 0.0, 11.0 / 1080.0 * t190 * t107);
        let tv3rho2lapl0 = 2.0 * rho[ip] * t288 + 4.0 * t186;
        v3rho2lapl[ip] += tv3rho2lapl0;
        let tv3rho2tau0 = 0.0;
        v3rho2tau[ip] += tv3rho2tau0;
        let t292 = t8 * t21 * t217;
        let t295 = t6 * t6;
        let t298 = t5 / t295 / t26;
        let t299 = t298 * t21;
        let t300 = t227 * t232;
        let t305 = piecewise3(t3, 0.0, -7.0 / 2880.0 * t292 * t193 + t299 * t300 * t170 / 1440.0);
        let tv3rhosigma20 = 2.0 * rho[ip] * t305 + 2.0 * t196;
        v3rhosigma2[ip] += tv3rhosigma20;
        let tv3rhosigmalapl0 = 0.0;
        v3rhosigmalapl[ip] += tv3rhosigmalapl0;
        let tv3rhosigmatau0 = 0.0;
        v3rhosigmatau[ip] += tv3rhosigmatau0;
        let t310 = piecewise3(t3, 0.0, -t183 * t198 / 360.0);
        let tv3rholapl20 = 2.0 * rho[ip] * t310 + 2.0 * t201;
        v3rholapl2[ip] += tv3rholapl20;
        let tv3rholapltau0 = 0.0;
        v3rholapltau[ip] += tv3rholapltau0;
        let tv3rhotau20 = 0.0;
        v3rhotau2[ip] += tv3rhotau20;
        let t317 = piecewise3(t3, 0.0, -t299 * t138 * t232 * t44 / 3840.0);
        let tv3sigma30 = 2.0 * rho[ip] * t317;
        v3sigma3[ip] += tv3sigma30;
        let tv3sigma2lapl0 = 0.0;
        v3sigma2lapl[ip] += tv3sigma2lapl0;
        let tv3sigma2tau0 = 0.0;
        v3sigma2tau[ip] += tv3sigma2tau0;
        let tv3sigmalapl20 = 0.0;
        v3sigmalapl2[ip] += tv3sigmalapl20;
        let tv3sigmalapltau0 = 0.0;
        v3sigmalapltau[ip] += tv3sigmalapltau0;
        let tv3sigmatau20 = 0.0;
        v3sigmatau2[ip] += tv3sigmatau20;
        let tv3lapl30 = 0.0;
        v3lapl3[ip] += tv3lapl30;
        let tv3lapl2tau0 = 0.0;
        v3lapl2tau[ip] += tv3lapl2tau0;
        let tv3lapltau20 = 0.0;
        v3lapltau2[ip] += tv3lapltau20;
        let tv3tau30 = 0.0;
        v3tau3[ip] += tv3tau30;
        let t334 = 1.0 / t23 / t166;
        let t344 = 1.0 / t22 / t225 / rho[ip];
        let t350 = 1.0 / t225 / t77;
        let t355 = t131 * t131;
        let t356 = t355 * t234;
        let t357 = t134 * t134;
        let t364 = t30 * t32 * t44;
        let t375 = piecewise3(t3, 0.0, -14.0 / 135.0 * t8 * t21 * t54 * t58 + 8.0 / 45.0 * t8 * t206 * t83 - t8 * t115 * t149 / 5.0 + 2.0 / 5.0 * t8 * t64 * t246 + 3.0 / 20.0 * t8 * t24 * (13090.0 / 729.0 * t30 * t33 * t334 - 2618.0 / 243.0 * t41 * t33 * t334 * t44 + 1958.0 / 729.0 * t133 * t135 * t344 * t44 - 176.0 / 243.0 * t235 * t236 * t350 * t44 + 8.0 / 2187.0 * t356 * t357 / t23 / t225 / t166 * t364 + 1235.0 / 729.0 * t49 * t51 * t138));
        let tv4rho40 = 2.0 * rho[ip] * t375 + 8.0 * t251;
        v4rho4[ip] += tv4rho40;
        let t388 = t32 * t217;
        let t414 = piecewise3(t3, 0.0, 2.0 / 45.0 * t8 * t206 * t97 - t8 * t115 * t174 / 10.0 + 3.0 / 10.0 * t8 * t64 * t277 + 3.0 / 20.0 * t8 * t24 * (-770.0 / 243.0 * t30 * t388 + 154.0 / 81.0 * t41 * t388 * t44 - 341.0 / 486.0 * t133 * t31 * t227 * t170 + 19.0 / 81.0 * t235 * t238 * t134 * t44 - t356 / t23 / t225 / t143 * t236 * t364 / 729.0));
        let tv4rho3sigma0 = 2.0 * rho[ip] * t414 + 6.0 * t282;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t420 = piecewise3(t3, 0.0, -77.0 / 1620.0 * t292 * t107);
        let tv4rho3lapl0 = 2.0 * rho[ip] * t420 + 6.0 * t288;
        v4rho3lapl[ip] += tv4rho3lapl0;
        let tv4rho3tau0 = 0.0;
        v4rho3tau[ip] += tv4rho3tau0;
        let t437 = t29 * t32 * t44;
        let t442 = piecewise3(t3, 0.0, 119.0 / 8640.0 * t8 * t21 * t334 * t193 - 13.0 / 1440.0 * t299 * t344 * t232 * t170 + t298 * t21 * t350 * t355 * t134 * t25 * t437 / 12960.0);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t442 + 4.0 * t305;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let tv4rho2sigmalapl0 = 0.0;
        v4rho2sigmalapl[ip] += tv4rho2sigmalapl0;
        let tv4rho2sigmatau0 = 0.0;
        v4rho2sigmatau[ip] += tv4rho2sigmatau0;
        let t448 = piecewise3(t3, 0.0, 11.0 / 1080.0 * t190 * t198);
        let tv4rho2lapl20 = 2.0 * rho[ip] * t448 + 4.0 * t310;
        v4rho2lapl2[ip] += tv4rho2lapl20;
        let tv4rho2lapltau0 = 0.0;
        v4rho2lapltau[ip] += tv4rho2lapltau0;
        let tv4rho2tau20 = 0.0;
        v4rho2tau2[ip] += tv4rho2tau20;
        let t462 = piecewise3(t3, 0.0, 11.0 / 5760.0 * t299 * t300 * t44 - t298 * t21 * t238 * t355 * t30 * t33 * t44 / 34560.0);
        let tv4rhosigma30 = 2.0 * rho[ip] * t462 + 2.0 * t317;
        v4rhosigma3[ip] += tv4rhosigma30;
        let tv4rhosigma2lapl0 = 0.0;
        v4rhosigma2lapl[ip] += tv4rhosigma2lapl0;
        let tv4rhosigma2tau0 = 0.0;
        v4rhosigma2tau[ip] += tv4rhosigma2tau0;
        let tv4rhosigmalapl20 = 0.0;
        v4rhosigmalapl2[ip] += tv4rhosigmalapl20;
        let tv4rhosigmalapltau0 = 0.0;
        v4rhosigmalapltau[ip] += tv4rhosigmalapltau0;
        let tv4rhosigmatau20 = 0.0;
        v4rhosigmatau2[ip] += tv4rhosigmatau20;
        let tv4rholapl30 = 0.0;
        v4rholapl3[ip] += tv4rholapl30;
        let tv4rholapl2tau0 = 0.0;
        v4rholapl2tau[ip] += tv4rholapl2tau0;
        let tv4rholapltau20 = 0.0;
        v4rholapltau2[ip] += tv4rholapltau20;
        let tv4rhotau30 = 0.0;
        v4rhotau3[ip] += tv4rhotau30;
        let t471 = piecewise3(t3, 0.0, t298 * t21 * t272 * t355 * t25 * t437 / 92160.0);
        let tv4sigma40 = 2.0 * rho[ip] * t471;
        v4sigma4[ip] += tv4sigma40;
        let tv4sigma3lapl0 = 0.0;
        v4sigma3lapl[ip] += tv4sigma3lapl0;
        let tv4sigma3tau0 = 0.0;
        v4sigma3tau[ip] += tv4sigma3tau0;
        let tv4sigma2lapl20 = 0.0;
        v4sigma2lapl2[ip] += tv4sigma2lapl20;
        let tv4sigma2lapltau0 = 0.0;
        v4sigma2lapltau[ip] += tv4sigma2lapltau0;
        let tv4sigma2tau20 = 0.0;
        v4sigma2tau2[ip] += tv4sigma2tau20;
        let tv4sigmalapl30 = 0.0;
        v4sigmalapl3[ip] += tv4sigmalapl30;
        let tv4sigmalapl2tau0 = 0.0;
        v4sigmalapl2tau[ip] += tv4sigmalapl2tau0;
        let tv4sigmalapltau20 = 0.0;
        v4sigmalapltau2[ip] += tv4sigmalapltau20;
        let tv4sigmatau30 = 0.0;
        v4sigmatau3[ip] += tv4sigmatau30;
        let tv4lapl40 = 0.0;
        v4lapl4[ip] += tv4lapl40;
        let tv4lapl3tau0 = 0.0;
        v4lapl3tau[ip] += tv4lapl3tau0;
        let tv4lapl2tau20 = 0.0;
        v4lapl2tau2[ip] += tv4lapl2tau20;
        let tv4lapltau30 = 0.0;
        v4lapltau3[ip] += tv4lapltau30;
        let tv4tau40 = 0.0;
        v4tau4[ip] += tv4tau40;
    }
}
