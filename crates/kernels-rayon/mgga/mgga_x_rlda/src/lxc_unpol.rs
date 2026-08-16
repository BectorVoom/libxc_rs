//! MGGA_X_RLDA lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rlda.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_rlda_lxc_unpol(
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
    param_prefactor: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRTPI;
        let t5 = t4 * t4;
        let t6 = 1.0 <= zeta_threshold;
        let t7 = zeta_threshold - 1.0;
        let t9 = piecewise5(t6, t7, t6, -t7, 0.0);
        let t10 = 1.0 + t9;
        let t12 = pow_1_3(zeta_threshold);
        let t14 = pow_1_3(t10);
        let t16 = piecewise3(t10 <= zeta_threshold, t12 * zeta_threshold, t14 * t10);
        let t17 = t5 * t16;
        let t18 = pow_1_3(rho[ip]);
        let t21 = pow_1_3(1.0 / M_PI);
        let t22 = 1.0 / t21;
        let t23 = param_prefactor * t22;
        let t24 = M_CBRT4;
        let t25 = M_CBRT2;
        let t26 = t25 * t25;
        let t27 = tau[ip] * t26;
        let t28 = t18 * t18;
        let t30 = 1.0 / t28 / rho[ip];
        let t33 = lapl[ip] * t26;
        let t36 = 2.0 * t27 * t30 - t33 * t30 / 4.0;
        let t39 = t23 * t24 / t36;
        let t42 = piecewise3(t3, 0.0, -15.0 / 16.0 * t17 * t18 * t39);
        let tzk0 = 2.0 * t42;
        zk[ip] += tzk0;
        let t43 = 1.0 / t28;
        let t48 = t17 * t18 * param_prefactor;
        let t49 = t22 * t24;
        let t50 = t36 * t36;
        let t51 = 1.0 / t50;
        let t52 = rho[ip] * rho[ip];
        let t54 = 1.0 / t28 / t52;
        let t59 = -10.0 / 3.0 * t27 * t54 + 5.0 / 12.0 * t33 * t54;
        let t61 = t49 * t51 * t59;
        let t65 = piecewise3(t3, 0.0, -5.0 / 16.0 * t17 * t43 * t39 + 15.0 / 16.0 * t48 * t61);
        let tvrho0 = 2.0 * rho[ip] * t65 + 2.0 * t42;
        vrho[ip] += tvrho0;
        let tvsigma0 = 0.0;
        vsigma[ip] += tvsigma0;
        let t70 = 1.0 / t18 / rho[ip] * param_prefactor;
        let t71 = t17 * t70;
        let t73 = t49 * t51 * t26;
        let t74 = t71 * t73;
        let t76 = piecewise3(t3, 0.0, -15.0 / 64.0 * t74);
        let tvlapl0 = 2.0 * rho[ip] * t76;
        vlapl[ip] += tvlapl0;
        let t79 = piecewise3(t3, 0.0, 15.0 / 8.0 * t74);
        let tvtau0 = 2.0 * rho[ip] * t79;
        vtau[ip] += tvtau0;
        let t86 = t17 * t43 * param_prefactor;
        let t90 = 1.0 / t50 / t36;
        let t91 = t59 * t59;
        let t93 = t49 * t90 * t91;
        let t96 = t52 * rho[ip];
        let t98 = 1.0 / t28 / t96;
        let t103 = 80.0 / 9.0 * t27 * t98 - 10.0 / 9.0 * t33 * t98;
        let t105 = t49 * t51 * t103;
        let t109 = piecewise3(t3, 0.0, 5.0 / 24.0 * t17 * t30 * t39 + 5.0 / 8.0 * t86 * t61 - 15.0 / 8.0 * t48 * t93 + 15.0 / 16.0 * t48 * t105);
        let tv2rho20 = 2.0 * rho[ip] * t109 + 4.0 * t65;
        v2rho2[ip] += tv2rho20;
        let tv2rhosigma0 = 0.0;
        v2rhosigma[ip] += tv2rhosigma0;
        let t115 = t17 / t18 / t52 * param_prefactor;
        let t116 = t115 * t73;
        let t118 = t90 * t26;
        let t120 = t49 * t118 * t59;
        let t121 = t71 * t120;
        let t124 = piecewise3(t3, 0.0, 5.0 / 16.0 * t116 + 15.0 / 32.0 * t121);
        let tv2rholapl0 = 2.0 * rho[ip] * t124 + 2.0 * t76;
        v2rholapl[ip] += tv2rholapl0;
        let t130 = piecewise3(t3, 0.0, -5.0 / 2.0 * t116 - 15.0 / 4.0 * t121);
        let tv2rhotau0 = 2.0 * rho[ip] * t130 + 2.0 * t79;
        v2rhotau[ip] += tv2rhotau0;
        let tv2sigma20 = 0.0;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let tv2sigmatau0 = 0.0;
        v2sigmatau[ip] += tv2sigmatau0;
        let t135 = t17 / t96 * param_prefactor;
        let t137 = t49 * t90 * t25;
        let t138 = t135 * t137;
        let t140 = piecewise3(t3, 0.0, -15.0 / 64.0 * t138);
        let tv2lapl20 = 2.0 * rho[ip] * t140;
        v2lapl2[ip] += tv2lapl20;
        let t143 = piecewise3(t3, 0.0, 15.0 / 8.0 * t138);
        let tv2lapltau0 = 2.0 * rho[ip] * t143;
        v2lapltau[ip] += tv2lapltau0;
        let t146 = piecewise3(t3, 0.0, -15.0 * t138);
        let tv2tau20 = 2.0 * rho[ip] * t146;
        v2tau2[ip] += tv2tau20;
        let t153 = t17 * t30 * param_prefactor;
        let t160 = t50 * t50;
        let t161 = 1.0 / t160;
        let t162 = t91 * t59;
        let t164 = t49 * t161 * t162;
        let t167 = t90 * t59;
        let t169 = t49 * t167 * t103;
        let t172 = t52 * t52;
        let t174 = 1.0 / t28 / t172;
        let t179 = -880.0 / 27.0 * t27 * t174 + 110.0 / 27.0 * t33 * t174;
        let t181 = t49 * t51 * t179;
        let t185 = piecewise3(t3, 0.0, -25.0 / 72.0 * t17 * t54 * t39 - 5.0 / 8.0 * t153 * t61 - 15.0 / 8.0 * t86 * t93 + 15.0 / 16.0 * t86 * t105 + 45.0 / 8.0 * t48 * t164 - 45.0 / 8.0 * t48 * t169 + 15.0 / 16.0 * t48 * t181);
        let tv3rho30 = 2.0 * rho[ip] * t185 + 6.0 * t109;
        v3rho3[ip] += tv3rho30;
        let tv3rho2sigma0 = 0.0;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t192 = t17 / t18 / t96 * param_prefactor;
        let t193 = t192 * t73;
        let t195 = t115 * t120;
        let t199 = t49 * t161 * t26 * t91;
        let t200 = t71 * t199;
        let t203 = t49 * t118 * t103;
        let t204 = t71 * t203;
        let t207 = piecewise3(t3, 0.0, -35.0 / 48.0 * t193 - 5.0 / 4.0 * t195 - 45.0 / 32.0 * t200 + 15.0 / 32.0 * t204);
        let tv3rho2lapl0 = 2.0 * rho[ip] * t207 + 4.0 * t124;
        v3rho2lapl[ip] += tv3rho2lapl0;
        let t216 = piecewise3(t3, 0.0, 35.0 / 6.0 * t193 + 10.0 * t195 + 45.0 / 4.0 * t200 - 15.0 / 4.0 * t204);
        let tv3rho2tau0 = 2.0 * rho[ip] * t216 + 4.0 * t130;
        v3rho2tau[ip] += tv3rho2tau0;
        let tv3rhosigma20 = 0.0;
        v3rhosigma2[ip] += tv3rhosigma20;
        let tv3rhosigmalapl0 = 0.0;
        v3rhosigmalapl[ip] += tv3rhosigmalapl0;
        let tv3rhosigmatau0 = 0.0;
        v3rhosigmatau[ip] += tv3rhosigmatau0;
        let t221 = t17 / t172 * param_prefactor;
        let t223 = t161 * t25;
        let t225 = t49 * t223 * t59;
        let t227 = t135 * t225 + t221 * t137;
        let t229 = piecewise3(t3, 0.0, 45.0 / 64.0 * t227);
        let tv3rholapl20 = 2.0 * rho[ip] * t229 + 2.0 * t140;
        v3rholapl2[ip] += tv3rholapl20;
        let t234 = piecewise3(t3, 0.0, -45.0 / 8.0 * t227);
        let tv3rholapltau0 = 2.0 * rho[ip] * t234 + 2.0 * t143;
        v3rholapltau[ip] += tv3rholapltau0;
        let t238 = piecewise3(t3, 0.0, 45.0 * t227);
        let tv3rhotau20 = 2.0 * rho[ip] * t238 + 2.0 * t146;
        v3rhotau2[ip] += tv3rhotau20;
        let tv3sigma30 = 0.0;
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
        let t242 = t24 * t161;
        let t243 = t23 * t242;
        let t244 = t17 * t174 * t243;
        let t246 = piecewise3(t3, 0.0, -45.0 / 128.0 * t244);
        let tv3lapl30 = 2.0 * rho[ip] * t246;
        v3lapl3[ip] += tv3lapl30;
        let t249 = piecewise3(t3, 0.0, 45.0 / 16.0 * t244);
        let tv3lapl2tau0 = 2.0 * rho[ip] * t249;
        v3lapl2tau[ip] += tv3lapl2tau0;
        let t252 = piecewise3(t3, 0.0, -45.0 / 2.0 * t244);
        let tv3lapltau20 = 2.0 * rho[ip] * t252;
        v3lapltau2[ip] += tv3lapltau20;
        let t255 = piecewise3(t3, 0.0, 180.0 * t244);
        let tv3tau30 = 2.0 * rho[ip] * t255;
        v3tau3[ip] += tv3tau30;
        let t276 = 1.0 / t160 / t36;
        let t277 = t91 * t91;
        let t287 = t103 * t103;
        let t296 = t172 * rho[ip];
        let t298 = 1.0 / t28 / t296;
        let t308 = 25.0 / 27.0 * t17 * t98 * t39 + 25.0 / 18.0 * t17 * t54 * param_prefactor * t61 + 5.0 / 2.0 * t153 * t93 - 5.0 / 4.0 * t153 * t105 + 15.0 / 2.0 * t86 * t164 - 15.0 / 2.0 * t86 * t169 + 5.0 / 4.0 * t86 * t181 - 45.0 / 2.0 * t48 * t49 * t276 * t277 + 135.0 / 4.0 * t48 * t49 * t161 * t91 * t103 - 45.0 / 8.0 * t48 * t49 * t90 * t287 - 15.0 / 2.0 * t48 * t49 * t167 * t179 + 15.0 / 16.0 * t48 * t49 * t51 * (12320.0 / 81.0 * t27 * t298 - 1540.0 / 81.0 * t33 * t298);
        let t309 = piecewise3(t3, 0.0, t308);
        let tv4rho40 = 2.0 * rho[ip] * t309 + 8.0 * t185;
        v4rho4[ip] += tv4rho40;
        let tv4rho3sigma0 = 0.0;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t317 = t17 / t18 / t172 * param_prefactor * t73;
        let t319 = t192 * t120;
        let t321 = t115 * t199;
        let t323 = t115 * t203;
        let t325 = t276 * t26;
        let t328 = t71 * t49 * t325 * t162;
        let t335 = t17 * t70 * t22 * t242 * t26 * t59 * t103;
        let t339 = t71 * t49 * t118 * t179;
        let t342 = piecewise3(t3, 0.0, 175.0 / 72.0 * t317 + 35.0 / 8.0 * t319 + 45.0 / 8.0 * t321 - 15.0 / 8.0 * t323 + 45.0 / 8.0 * t328 - 135.0 / 32.0 * t335 + 15.0 / 32.0 * t339);
        let tv4rho3lapl0 = 2.0 * rho[ip] * t342 + 6.0 * t207;
        v4rho3lapl[ip] += tv4rho3lapl0;
        let t354 = piecewise3(t3, 0.0, -175.0 / 9.0 * t317 - 35.0 * t319 - 45.0 * t321 + 15.0 * t323 - 45.0 * t328 + 135.0 / 4.0 * t335 - 15.0 / 4.0 * t339);
        let tv4rho3tau0 = 2.0 * rho[ip] * t354 + 6.0 * t216;
        v4rho3tau[ip] += tv4rho3tau0;
        let tv4rho2sigma20 = 0.0;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let tv4rho2sigmalapl0 = 0.0;
        v4rho2sigmalapl[ip] += tv4rho2sigmalapl0;
        let tv4rho2sigmatau0 = 0.0;
        v4rho2sigmatau[ip] += tv4rho2sigmatau0;
        let t361 = t17 / t296 * param_prefactor * t137;
        let t363 = t221 * t225;
        let t368 = t135 * t49 * t276 * t25 * t91;
        let t372 = t135 * t49 * t223 * t103;
        let t375 = piecewise3(t3, 0.0, -45.0 / 16.0 * t361 - 135.0 / 32.0 * t363 - 45.0 / 16.0 * t368 + 45.0 / 64.0 * t372);
        let tv4rho2lapl20 = 2.0 * rho[ip] * t375 + 4.0 * t229;
        v4rho2lapl2[ip] += tv4rho2lapl20;
        let t384 = piecewise3(t3, 0.0, 45.0 / 2.0 * t361 + 135.0 / 4.0 * t363 + 45.0 / 2.0 * t368 - 45.0 / 8.0 * t372);
        let tv4rho2lapltau0 = 2.0 * rho[ip] * t384 + 4.0 * t234;
        v4rho2lapltau[ip] += tv4rho2lapltau0;
        let t393 = piecewise3(t3, 0.0, -180.0 * t361 - 270.0 * t363 - 180.0 * t368 + 45.0 * t372);
        let tv4rho2tau20 = 2.0 * rho[ip] * t393 + 4.0 * t238;
        v4rho2tau2[ip] += tv4rho2tau20;
        let tv4rhosigma30 = 0.0;
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
        let t397 = t17 * t298 * t243;
        let t403 = t17 * t174 * param_prefactor * t49 * t276 * t59;
        let t406 = piecewise3(t3, 0.0, 105.0 / 64.0 * t397 + 45.0 / 32.0 * t403);
        let tv4rholapl30 = 2.0 * rho[ip] * t406 + 2.0 * t246;
        v4rholapl3[ip] += tv4rholapl30;
        let t412 = piecewise3(t3, 0.0, -105.0 / 8.0 * t397 - 45.0 / 4.0 * t403);
        let tv4rholapl2tau0 = 2.0 * rho[ip] * t412 + 2.0 * t249;
        v4rholapl2tau[ip] += tv4rholapl2tau0;
        let t418 = piecewise3(t3, 0.0, 105.0 * t397 + 90.0 * t403);
        let tv4rholapltau20 = 2.0 * rho[ip] * t418 + 2.0 * t252;
        v4rholapltau2[ip] += tv4rholapltau20;
        let t424 = piecewise3(t3, 0.0, -840.0 * t397 - 720.0 * t403);
        let tv4rhotau30 = 2.0 * rho[ip] * t424 + 2.0 * t255;
        v4rhotau3[ip] += tv4rhotau30;
        let tv4sigma40 = 0.0;
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
        let t433 = t17 / t18 / t172 / t52 * param_prefactor * t49 * t325;
        let t435 = piecewise3(t3, 0.0, -45.0 / 128.0 * t433);
        let tv4lapl40 = 2.0 * rho[ip] * t435;
        v4lapl4[ip] += tv4lapl40;
        let t438 = piecewise3(t3, 0.0, 45.0 / 16.0 * t433);
        let tv4lapl3tau0 = 2.0 * rho[ip] * t438;
        v4lapl3tau[ip] += tv4lapl3tau0;
        let t441 = piecewise3(t3, 0.0, -45.0 / 2.0 * t433);
        let tv4lapl2tau20 = 2.0 * rho[ip] * t441;
        v4lapl2tau2[ip] += tv4lapl2tau20;
        let t444 = piecewise3(t3, 0.0, 180.0 * t433);
        let tv4lapltau30 = 2.0 * rho[ip] * t444;
        v4lapltau3[ip] += tv4lapltau30;
        let t447 = piecewise3(t3, 0.0, -1440.0 * t433);
        let tv4tau40 = 2.0 * rho[ip] * t447;
        v4tau4[ip] += tv4tau40;
    }
}
