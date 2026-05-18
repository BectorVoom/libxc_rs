//! MGGA_C_CS lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_cs.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_c_cs_lxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2rholapl: &mut Array<f64>,
    v2rhotau: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v2sigmalapl: &mut Array<f64>,
    v2sigmatau: &mut Array<f64>,
    v2lapl2: &mut Array<f64>,
    v2lapltau: &mut Array<f64>,
    v2tau2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rho2lapl: &mut Array<f64>,
    v3rho2tau: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3rhosigmalapl: &mut Array<f64>,
    v3rhosigmatau: &mut Array<f64>,
    v3rholapl2: &mut Array<f64>,
    v3rholapltau: &mut Array<f64>,
    v3rhotau2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v3sigma2lapl: &mut Array<f64>,
    v3sigma2tau: &mut Array<f64>,
    v3sigmalapl2: &mut Array<f64>,
    v3sigmalapltau: &mut Array<f64>,
    v3sigmatau2: &mut Array<f64>,
    v3lapl3: &mut Array<f64>,
    v3lapl2tau: &mut Array<f64>,
    v3lapltau2: &mut Array<f64>,
    v3tau3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho3lapl: &mut Array<f64>,
    v4rho3tau: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rho2sigmalapl: &mut Array<f64>,
    v4rho2sigmatau: &mut Array<f64>,
    v4rho2lapl2: &mut Array<f64>,
    v4rho2lapltau: &mut Array<f64>,
    v4rho2tau2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4rhosigma2lapl: &mut Array<f64>,
    v4rhosigma2tau: &mut Array<f64>,
    v4rhosigmalapl2: &mut Array<f64>,
    v4rhosigmalapltau: &mut Array<f64>,
    v4rhosigmatau2: &mut Array<f64>,
    v4rholapl3: &mut Array<f64>,
    v4rholapl2tau: &mut Array<f64>,
    v4rholapltau2: &mut Array<f64>,
    v4rhotau3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    v4sigma3lapl: &mut Array<f64>,
    v4sigma3tau: &mut Array<f64>,
    v4sigma2lapl2: &mut Array<f64>,
    v4sigma2lapltau: &mut Array<f64>,
    v4sigma2tau2: &mut Array<f64>,
    v4sigmalapl3: &mut Array<f64>,
    v4sigmalapl2tau: &mut Array<f64>,
    v4sigmalapltau2: &mut Array<f64>,
    v4sigmatau3: &mut Array<f64>,
    v4lapl4: &mut Array<f64>,
    v4lapl3tau: &mut Array<f64>,
    v4lapl2tau2: &mut Array<f64>,
    v4lapltau3: &mut Array<f64>,
    v4tau4: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = pow_1_3::<f64>(rho[ip]);
        let t3 = 1.0 / t2;
        let t5 = 1.0 + 0.34899999999999999998e0 * t3;
        let t6 = 1.0 / t5;
        let t8 = f64::exp(-0.2533e0 * t3);
        let t10 = zeta_threshold * zeta_threshold;
        let t11 = pow_1_3::<f64>(zeta_threshold);
        let t12 = t11 * t11;
        let t14 = piecewise3::<f64>(1.0 <= zeta_threshold, t12 * t10, 1.0);
        let t15 = M_CBRT2;
        let t16 = t14 * t15;
        let t17 = t15 * t15;
        let t18 = tau[ip] * t17;
        let t19 = t2 * t2;
        let t21 = 1.0 / t19 / rho[ip];
        let t23 = lapl[ip] * t17;
        let t29 = rho[ip] * rho[ip];
        let t31 = 1.0 / t19 / t29;
        let t36 = t16 * (t18 * t21 - t23 * t21 / 8.0) / 4.0 - sigma[ip] * t31 / 8.0 + lapl[ip] * t21 / 8.0;
        let t39 = 1.0 + 0.264e0 * t8 * t36;
        let tzk0 = -0.4918e-1 * t6 * t39;
        zk[ip] += tzk0;
        let t42 = t5 * t5;
        let t43 = 1.0 / t42;
        let t44 = t3 * t43;
        let t47 = rho[ip] * t6;
        let t49 = 1.0 / t2 / rho[ip];
        let t50 = t49 * t8;
        let t60 = t29 * rho[ip];
        let t62 = 1.0 / t19 / t60;
        let t67 = t16 * (-5.0 / 3.0 * t18 * t31 + 5.0 / 24.0 * t23 * t31) / 4.0 + sigma[ip] * t62 / 3.0 - 5.0 / 24.0 * lapl[ip] * t31;
        let t70 = 0.222904e-1 * t50 * t36 + 0.264e0 * t8 * t67;
        let tvrho0 = tzk0 - 0.57212733333333333332e-2 * t44 * t39 - 0.4918e-1 * t47 * t70;
        vrho[ip] += tvrho0;
        let t73 = t21 * t6;
        let tvsigma0 = 0.162294e-2 * t73 * t8;
        vsigma[ip] += tvsigma0;
        let t78 = -t14 * t21 / 16.0 + t21 / 8.0;
        let t79 = t8 * t78;
        let tvlapl0 = -0.1298352e-1 * t47 * t79;
        vlapl[ip] += tvlapl0;
        let t84 = t8 * t14;
        let tvtau0 = -0.649176e-2 / t19 * t6 * t84;
        vtau[ip] += tvtau0;
        let t87 = t43 * t39;
        let t93 = 1.0 / t42 / t5;
        let t94 = t21 * t93;
        let t100 = 1.0 / t2 / t29;
        let t101 = t100 * t8;
        let t104 = t31 * t8;
        let t116 = t29 * t29;
        let t118 = 1.0 / t19 / t116;
        let t123 = t16 * (40.0 / 9.0 * t18 * t62 - 5.0 / 9.0 * t23 * t62) / 4.0 - 11.0 / 9.0 * sigma[ip] * t118 + 5.0 / 9.0 * lapl[ip] * t62;
        let t126 = -0.29720533333333333333e-1 * t101 * t36 + 0.18820527733333333333e-2 * t104 * t36 + 0.445808e-1 * t50 * t67 + 0.264e0 * t8 * t123;
        let tv2rho20 = -0.38141822222222222221e-2 * t87 * t49 - 0.9836e-1 * t6 * t70 - 0.13311495955555555555e-2 * t94 * t39 - 0.11442546666666666666e-1 * t44 * t70 - 0.4918e-1 * t47 * t126;
        v2rho2[ip] += tv2rho20;
        let t129 = t31 * t6;
        let t132 = 1.0 / t60;
        let t133 = t132 * t43;
        let t136 = t132 * t6;
        let tv2rhosigma0 = -0.27049e-2 * t129 * t8 + 0.18880201999999999999e-3 * t133 * t8 + 0.137030234e-3 * t136 * t8;
        v2rhosigma[ip] += tv2rhosigma0;
        let t139 = t6 * t8;
        let t144 = t3 * t6;
        let t150 = 5.0 / 48.0 * t14 * t31 - 5.0 / 24.0 * t31;
        let t151 = t8 * t150;
        let tv2rholapl0 = -0.1298352e-1 * t139 * t78 - 0.151041616e-2 * t44 * t79 - 0.1096241872e-2 * t144 * t79 - 0.1298352e-1 * t47 * t151;
        v2rholapl[ip] += tv2rholapl0;
        let t156 = 1.0 / t29;
        let tv2rhotau0 = 0.432784e-2 * t73 * t84 - 0.75520807999999999998e-3 * t156 * t43 * t84 - 0.548120936e-3 * t156 * t6 * t84;
        v2rhotau[ip] += tv2rhotau0;
        let tv2sigma20 = 0.0;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let tv2sigmatau0 = 0.0;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let tv2tau20 = 0.0;
        v2tau2[ip] += tv2tau20;
        let t163 = t93 * t39;
        let t166 = t43 * t70;
        let t173 = t42 * t42;
        let t174 = 1.0 / t173;
        let t175 = t132 * t174;
        let t183 = 1.0 / t2 / t60;
        let t184 = t183 * t8;
        let t187 = t62 * t8;
        let t192 = 1.0 / t116;
        let t193 = t192 * t8;
        let t207 = t116 * rho[ip];
        let t209 = 1.0 / t19 / t207;
        let t214 = t16 * (-440.0 / 27.0 * t18 * t118 + 55.0 / 27.0 * t23 * t118) / 4.0 + 154.0 / 27.0 * sigma[ip] * t209 - 55.0 / 27.0 * lapl[ip] * t118;
        let t217 = 0.6934791111111111111e-1 * t184 * t36 - 0.75282110933333333332e-2 * t187 * t36 - 0.891616e-1 * t101 * t67 + 0.15890798916177777777e-3 * t193 * t36 + 0.564615832e-2 * t104 * t67 + 0.668712e-1 * t50 * t123 + 0.264e0 * t8 * t214;
        let tv3rho30 = 0.13311495955555555556e-2 * t163 * t31 - 0.11442546666666666666e-1 * t166 * t49 + 0.50855762962962962961e-2 * t87 * t100 - 0.14754e0 * t6 * t126 - 0.46457120884888888886e-3 * t175 * t39 - 0.39934487866666666664e-2 * t94 * t70 - 0.17163819999999999999e-1 * t44 * t126 - 0.4918e-1 * t47 * t217;
        v3rho3[ip] += tv3rho30;
        let t220 = t62 * t6;
        let t223 = t192 * t43;
        let t226 = t192 * t6;
        let t230 = 1.0 / t2 / t116;
        let t231 = t230 * t93;
        let t234 = t230 * t43;
        let t237 = t230 * t6;
        let tv3rho2sigma0 = 0.72130666666666666667e-2 * t220 * t8 - 0.88107609333333333329e-3 * t223 * t8 - 0.63947442533333333333e-3 * t226 * t8 + 0.4392793665333333333e-4 * t231 * t8 + 0.31882367777333333332e-4 * t234 * t8 + 0.11569919424066666667e-4 * t237 * t8;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t240 = t43 * t8;
        let t244 = t6 * t49;
        let t251 = t21 * t43;
        let t263 = -5.0 / 18.0 * t14 * t62 + 5.0 / 9.0 * t62;
        let t264 = t8 * t263;
        let tv3rho2lapl0 = -0.10069441066666666667e-2 * t240 * t78 * t49 - 0.73082791466666666667e-3 * t244 * t79 - 0.2596704e-1 * t139 * t150 - 0.35142349322666666666e-3 * t94 * t79 - 0.25505894221866666666e-3 * t251 * t79 - 0.302083232e-2 * t44 * t151 - 0.92559355392533333333e-4 * t73 * t79 - 0.2192483744e-2 * t144 * t151 - 0.1298352e-1 * t47 * t264;
        v3rho2lapl[ip] += tv3rho2lapl0;
        let tv3rho2tau0 = -0.72130666666666666667e-2 * t129 * t84 + 0.20138882133333333333e-2 * t133 * t84 + 0.14616558293333333333e-2 * t136 * t84 - 0.17571174661333333332e-3 * t183 * t93 * t84 - 0.12752947110933333333e-3 * t183 * t43 * t84 - 0.46279677696266666666e-4 * t183 * t6 * t84;
        v3rho2tau[ip] += tv3rho2tau0;
        let tv3rhosigma20 = 0.0;
        v3rhosigma2[ip] += tv3rhosigma20;
        let tv3rhosigmalapl0 = 0.0;
        v3rhosigmalapl[ip] += tv3rhosigmalapl0;
        let tv3rhosigmatau0 = 0.0;
        v3rhosigmatau[ip] += tv3rhosigmatau0;
        let tv3rholapl20 = 0.0;
        v3rholapl2[ip] += tv3rholapl20;
        let tv3rholapltau0 = 0.0;
        v3rholapltau[ip] += tv3rholapltau0;
        let tv3rhotau20 = 0.0;
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
        let tv3lapl30 = 0.0;
        v3lapl3[ip] += tv3lapl30;
        let tv3lapl2tau0 = 0.0;
        v3lapl2tau[ip] += tv3lapl2tau0;
        let tv3lapltau20 = 0.0;
        v3lapltau2[ip] += tv3lapltau20;
        let tv3tau30 = 0.0;
        v3tau3[ip] += tv3tau30;
        let t318 = 1.0 / t207;
        let t327 = 1.0 / t2 / t207;
        let t354 = -0.2311597037037037037e0 * t230 * t8 * t36 + 0.3345871597037037037e-1 * t118 * t8 * t36 + 0.27739164444444444444e0 * t184 * t67 - 0.12712639132942222222e-2 * t318 * t8 * t36 - 0.30112844373333333333e-1 * t187 * t67 - 0.1783232e0 * t101 * t123 + 0.13417131218226103703e-4 * t327 * t8 * t36 + 0.6356319566471111111e-3 * t193 * t67 + 0.1129231664e-1 * t104 * t123 + 0.891616e-1 * t50 * t214 + 0.264e0 * t8 * (t16 * (6160.0 / 81.0 * t18 * t209 - 770.0 / 81.0 * t23 * t209) / 4.0 - 2618.0 / 81.0 * sigma[ip] / t19 / t116 / t29 + 770.0 / 81.0 * lapl[ip] * t209);
        let tv4rho40 = 0.18582848353955555555e-2 * t174 * t39 * t192 + 0.5324598382222222222e-2 * t93 * t70 * t31 - 0.236648816987654321e-2 * t163 * t62 - 0.22885093333333333333e-1 * t43 * t126 * t49 + 0.20342305185185185184e-1 * t166 * t100 - 0.11866344691358024691e-1 * t87 * t183 - 0.19672e0 * t6 * t217 - 0.21618046918434962961e-3 * t230 / t173 / t5 * t39 - 0.18582848353955555554e-2 * t175 * t70 - 0.79868975733333333327e-2 * t94 * t126 - 0.22885093333333333332e-1 * t44 * t217 - 0.4918e-1 * t47 * t354;
        v4rho4[ip] += tv4rho40;
        let t357 = t118 * t6;
        let tv4rho3sigma0 = -0.26447911111111111111e-1 * t357 * t8 + 0.43634244622222222221e-2 * t318 * t43 * t8 + 0.31669209635555555555e-2 * t318 * t6 * t8 - 0.39535142987999999997e-3 * t327 * t93 * t8 - 0.28694130999599999999e-3 * t327 * t43 * t8 - 0.1041292748166e-3 * t327 * t6 * t8 + 0.15330849892013333332e-4 * t209 * t174 * t8 + 0.11126946354289333332e-4 * t209 * t93 * t8 + 0.40379018789992666666e-5 * t209 * t43 * t8 + 0.97688686337202888891e-6 * t209 * t6 * t8;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let tv4rho3lapl0 = -0.30208323199999999999e-2 * t240 * t150 * t49 + 0.35142349322666666665e-3 * t93 * t8 * t78 * t31 + 0.25505894221866666666e-3 * t43 * t31 * t79 - 0.2192483744e-2 * t244 * t151 + 0.92559355392533333334e-4 * t129 * t79 - 0.105427047968e-2 * t94 * t151 - 0.12264679913610666666e-3 * t175 * t79 - 0.89015570834314666663e-4 * t132 * t93 * t79 - 0.76517682665599999999e-3 * t251 * t151 - 0.32303215031994133332e-4 * t133 * t79 - 0.453124848e-2 * t44 * t264 - 0.2776780661776e-3 * t73 * t151 - 0.78150949069762311111e-5 * t136 * t79 - 0.3288725616e-2 * t144 * t264 - 0.1298352e-1 * t47 * t8 * (55.0 / 54.0 * t14 * t118 - 55.0 / 27.0 * t118) - 0.3895056e-1 * t139 * t263 + 0.13425921422222222223e-2 * t240 * t78 * t100 + 0.97443721955555555556e-3 * t6 * t100 * t79;
        v4rho3lapl[ip] += tv4rho3lapl0;
        let tv4rho3tau0 = 0.19234844444444444445e-1 * t220 * t84 - 0.68807847288888888888e-2 * t223 * t84 - 0.49939907502222222221e-2 * t226 * t84 + 0.10542704796799999999e-2 * t231 * t84 + 0.76517682665599999998e-3 * t234 * t84 + 0.2776780661776e-3 * t237 * t84 - 0.61323399568053333327e-4 * t118 * t174 * t84 - 0.44507785417157333331e-4 * t118 * t93 * t84 - 0.16151607515997066666e-4 * t118 * t43 * t84 - 0.39075474534881155555e-5 * t357 * t84;
        v4rho3tau[ip] += tv4rho3tau0;
        let tv4rho2sigma20 = 0.0;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let tv4rho2sigmalapl0 = 0.0;
        v4rho2sigmalapl[ip] += tv4rho2sigmalapl0;
        let tv4rho2sigmatau0 = 0.0;
        v4rho2sigmatau[ip] += tv4rho2sigmatau0;
        let tv4rho2lapl20 = 0.0;
        v4rho2lapl2[ip] += tv4rho2lapl20;
        let tv4rho2lapltau0 = 0.0;
        v4rho2lapltau[ip] += tv4rho2lapltau0;
        let tv4rho2tau20 = 0.0;
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
        let tv4rholapl30 = 0.0;
        v4rholapl3[ip] += tv4rholapl30;
        let tv4rholapl2tau0 = 0.0;
        v4rholapl2tau[ip] += tv4rholapl2tau0;
        let tv4rholapltau20 = 0.0;
        v4rholapltau2[ip] += tv4rholapltau20;
        let tv4rhotau30 = 0.0;
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
