//! GGA_K_PW86 lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_pw86.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_pw86_lxc_unpol(
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
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
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
        let t29 = t24 / t27;
        let t30 = M_CBRT2;
        let t31 = t30 * t30;
        let t32 = sigma[ip] * t31;
        let t33 = rho[ip] * rho[ip];
        let t35 = 1.0 / t22 / t33;
        let t39 = t24 * t24;
        let t42 = t39 / t26 / t25;
        let t43 = sigma[ip] * sigma[ip];
        let t44 = t43 * t30;
        let t45 = t33 * t33;
        let t46 = t45 * rho[ip];
        let t48 = 1.0 / t21 / t46;
        let t52 = t43 * sigma[ip];
        let t53 = t45 * t45;
        let t54 = 1.0 / t53;
        let t57 = 1.0 + 0.91999999999999999998e-1 * t29 * t32 * t35 + 0.321875e-1 * t42 * t44 * t48 + 0.35645771717653941627e-5 * t52 * t54;
        let t58 = f64::powf(t57, 1.0 / 15.0);
        let t62 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t58);
        let tzk0 = 2.0 * t62;
        zk[ip] += tzk0;
        let t63 = 1.0 / t21;
        let t68 = t7 * t20;
        let t69 = t58 * t58;
        let t70 = t69 * t69;
        let t72 = t70 * t70;
        let t73 = t72 * t70 * t69;
        let t74 = 1.0 / t73;
        let t75 = t22 * t74;
        let t76 = t33 * rho[ip];
        let t78 = 1.0 / t22 / t76;
        let t82 = t45 * t33;
        let t84 = 1.0 / t21 / t82;
        let t88 = t53 * rho[ip];
        let t89 = 1.0 / t88;
        let t92 = -0.24533333333333333333e0 * t29 * t32 * t78 - 0.17166666666666666667e0 * t42 * t44 * t84 - 0.28516617374123153302e-4 * t52 * t89;
        let t97 = piecewise3(t2, 0.0, t7 * t20 * t63 * t58 / 10.0 + t68 * t75 * t92 / 100.0);
        let tvrho0 = 2.0 * rho[ip] * t97 + 2.0 * t62;
        vrho[ip] += tvrho0;
        let t103 = sigma[ip] * t30;
        let t109 = 0.91999999999999999998e-1 * t29 * t31 * t35 + 0.64375e-1 * t42 * t103 * t48 + 0.10693731515296182488e-4 * t43 * t54;
        let t113 = piecewise3(t2, 0.0, t68 * t75 * t109 / 100.0);
        let tvsigma0 = 2.0 * rho[ip] * t113;
        vsigma[ip] += tvsigma0;
        let t117 = 1.0 / t21 / rho[ip];
        let t122 = t63 * t74;
        let t127 = 1.0 / t73 / t57;
        let t128 = t22 * t127;
        let t129 = t92 * t92;
        let t134 = 1.0 / t22 / t45;
        let t140 = 1.0 / t21 / t45 / t76;
        let t145 = 1.0 / t53 / t33;
        let t148 = 0.89955555555555555554e0 * t29 * t32 * t134 + 0.10872222222222222222e1 * t42 * t44 * t140 + 0.25664955636710837972e-3 * t52 * t145;
        let t153 = piecewise3(t2, 0.0, -t7 * t20 * t117 * t58 / 30.0 + t68 * t122 * t92 / 75.0 - 7.0 / 750.0 * t68 * t128 * t129 + t68 * t75 * t148 / 100.0);
        let tv2rho20 = 2.0 * rho[ip] * t153 + 4.0 * t97;
        v2rho2[ip] += tv2rho20;
        let t159 = t109 * t92;
        let t171 = -0.24533333333333333333e0 * t29 * t31 * t78 - 0.34333333333333333333e0 * t42 * t103 * t84 - 0.85549852122369459904e-4 * t43 * t89;
        let t176 = piecewise3(t2, 0.0, t68 * t122 * t109 / 150.0 - 7.0 / 750.0 * t68 * t128 * t159 + t68 * t75 * t171 / 100.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t176 + 2.0 * t113;
        v2rhosigma[ip] += tv2rhosigma0;
        let t179 = t109 * t109;
        let t188 = 0.64375e-1 * t42 * t30 * t48 + 0.21387463030592364976e-4 * sigma[ip] * t54;
        let t193 = piecewise3(t2, 0.0, -7.0 / 750.0 * t68 * t128 * t179 + t68 * t75 * t188 / 100.0);
        let tv2sigma20 = 2.0 * rho[ip] * t193;
        v2sigma2[ip] += tv2sigma20;
        let t197 = 1.0 / t21 / t33;
        let t202 = t117 * t74;
        let t206 = t63 * t127;
        let t213 = t57 * t57;
        let t215 = 1.0 / t73 / t213;
        let t216 = t22 * t215;
        let t217 = t129 * t92;
        let t221 = t92 * t148;
        let t226 = 1.0 / t22 / t46;
        let t231 = 1.0 / t21 / t53;
        let t236 = 1.0 / t53 / t76;
        let t239 = -0.41979259259259259259e1 * t29 * t32 * t226 - 0.79729629629629629628e1 * t42 * t44 * t231 - 0.25664955636710837972e-2 * t52 * t236;
        let t244 = piecewise3(t2, 0.0, 2.0 / 45.0 * t7 * t20 * t197 * t58 - t68 * t202 * t92 / 150.0 - 7.0 / 375.0 * t68 * t206 * t129 + t68 * t122 * t148 / 50.0 + 203.0 / 11250.0 * t68 * t216 * t217 - 7.0 / 250.0 * t68 * t128 * t221 + t68 * t75 * t239 / 100.0);
        let tv3rho30 = 2.0 * rho[ip] * t244 + 6.0 * t153;
        v3rho3[ip] += tv3rho30;
        let t257 = t109 * t129;
        let t261 = t171 * t92;
        let t265 = t109 * t148;
        let t277 = 0.89955555555555555554e0 * t29 * t31 * t134 + 0.21744444444444444444e1 * t42 * t103 * t140 + 0.76994866910132513914e-3 * t43 * t145;
        let t282 = piecewise3(t2, 0.0, -t68 * t202 * t109 / 450.0 - 14.0 / 1125.0 * t68 * t206 * t159 + t68 * t122 * t171 / 75.0 + 203.0 / 11250.0 * t68 * t216 * t257 - 7.0 / 375.0 * t68 * t128 * t261 - 7.0 / 750.0 * t68 * t128 * t265 + t68 * t75 * t277 / 100.0);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t282 + 4.0 * t176;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t288 = t179 * t92;
        let t292 = t109 * t171;
        let t299 = t188 * t92;
        let t308 = -0.34333333333333333333e0 * t42 * t30 * t84 - 0.17109970424473891981e-3 * sigma[ip] * t89;
        let t313 = piecewise3(t2, 0.0, -7.0 / 1125.0 * t68 * t206 * t179 + 203.0 / 11250.0 * t68 * t216 * t288 - 7.0 / 375.0 * t68 * t128 * t292 + t68 * t122 * t188 / 150.0 - 7.0 / 750.0 * t68 * t128 * t299 + t68 * t75 * t308 / 100.0);
        let tv3rhosigma20 = 2.0 * rho[ip] * t313 + 2.0 * t193;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t316 = t179 * t109;
        let t320 = t109 * t188;
        let t324 = t4 * t20;
        let t329 = piecewise3(t2, 0.0, 203.0 / 11250.0 * t68 * t216 * t316 - 7.0 / 250.0 * t68 * t128 * t320 + 0.98406949358906462839e-6 * t324 * t140 * t74);
        let tv3sigma30 = 2.0 * rho[ip] * t329;
        v3sigma3[ip] += tv3sigma30;
        let t338 = t197 * t74;
        let t342 = t117 * t127;
        let t349 = t63 * t215;
        let t362 = t22 / t73 / t213 / t57;
        let t363 = t129 * t129;
        let t371 = t148 * t148;
        let t397 = -14.0 / 135.0 * t7 * t20 / t21 / t76 * t58 + 8.0 / 675.0 * t68 * t338 * t92 + 14.0 / 1125.0 * t68 * t342 * t129 - t68 * t202 * t148 / 75.0 + 812.0 / 16875.0 * t68 * t349 * t217 - 28.0 / 375.0 * t68 * t206 * t221 + 2.0 / 75.0 * t68 * t122 * t239 - 4466.0 / 84375.0 * t68 * t362 * t363 + 203.0 / 1875.0 * t68 * t216 * t129 * t148 - 7.0 / 250.0 * t68 * t128 * t371 - 14.0 / 375.0 * t68 * t128 * t92 * t239 + t68 * t75 * (0.23788246913580246913e2 * t29 * t32 / t22 / t82 + 0.66441358024691358023e2 * t42 * t44 / t21 / t88 + 0.28231451200381921769e-1 * t52 / t53 / t45) / 100.0;
        let t398 = piecewise3(t2, 0.0, t397);
        let tv4rho40 = 2.0 * rho[ip] * t398 + 8.0 * t244;
        v4rho4[ip] += tv4rho40;
        let t427 = t7 * t23;
        let t428 = t215 * t109;
        let t460 = -t68 * t202 * t171 / 150.0 + t68 * t122 * t277 / 50.0 + t68 * t75 * (-0.41979259259259259259e1 * t29 * t31 * t226 - 0.15945925925925925926e2 * t42 * t103 * t231 - 0.76994866910132513914e-2 * t43 * t236) / 100.0 + 203.0 / 5625.0 * t68 * t349 * t257 - 4466.0 / 84375.0 * t68 * t362 * t109 * t217 + 203.0 / 3750.0 * t427 * t428 * t221 + 203.0 / 3750.0 * t68 * t216 * t171 * t129 - 7.0 / 250.0 * t68 * t128 * t277 * t92 - 7.0 / 250.0 * t68 * t128 * t171 * t148 - 7.0 / 750.0 * t68 * t128 * t109 * t239 + 2.0 / 675.0 * t68 * t338 * t109 + 7.0 / 1125.0 * t68 * t342 * t159 - 14.0 / 375.0 * t68 * t206 * t261 - 7.0 / 375.0 * t68 * t206 * t265;
        let t461 = piecewise3(t2, 0.0, t460);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t461 + 6.0 * t282;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t485 = t171 * t171;
        let t523 = 7.0 / 3375.0 * t68 * t342 * t179 + 406.0 / 16875.0 * t68 * t349 * t288 - 28.0 / 1125.0 * t68 * t206 * t292 - 4466.0 / 84375.0 * t68 * t362 * t179 * t129 + 406.0 / 5625.0 * t427 * t428 * t261 + 203.0 / 11250.0 * t68 * t216 * t179 * t148 - 7.0 / 375.0 * t68 * t128 * t485 - 7.0 / 375.0 * t68 * t128 * t109 * t277 - t68 * t202 * t188 / 450.0 - 14.0 / 1125.0 * t68 * t206 * t299 + t68 * t122 * t308 / 75.0 + 203.0 / 11250.0 * t68 * t216 * t188 * t129 - 7.0 / 375.0 * t68 * t128 * t308 * t92 - 7.0 / 750.0 * t68 * t128 * t188 * t148 + t68 * t75 * (0.21744444444444444444e1 * t42 * t30 * t140 + 0.15398973382026502783e-2 * sigma[ip] * t145) / 100.0;
        let t524 = piecewise3(t2, 0.0, t523);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t524 + 4.0 * t313;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t555 = t140 * t127;
        let t560 = piecewise3(t2, 0.0, 203.0 / 16875.0 * t68 * t349 * t316 - 4466.0 / 84375.0 * t68 * t362 * t316 * t92 + 203.0 / 3750.0 * t68 * t216 * t179 * t171 - 7.0 / 375.0 * t68 * t206 * t320 + 203.0 / 3750.0 * t427 * t428 * t299 - 7.0 / 250.0 * t68 * t128 * t171 * t188 - 7.0 / 250.0 * t68 * t128 * t109 * t308 - 0.72165096196531406082e-5 * t324 * t231 * t74 - 0.9184648606831269865e-6 * t324 * t555 * t92);
        let tv4rhosigma30 = 2.0 * rho[ip] * t560 + 2.0 * t329;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t563 = t179 * t179;
        let t571 = t188 * t188;
        let t579 = piecewise3(t2, 0.0, -4466.0 / 84375.0 * t68 * t362 * t563 + 203.0 / 1875.0 * t68 * t216 * t179 * t188 - 7.0 / 250.0 * t68 * t128 * t571 - 0.3673859442732507946e-5 * t324 * t555 * t109);
        let tv4sigma40 = 2.0 * rho[ip] * t579;
        v4sigma4[ip] += tv4sigma40;
    }
}
