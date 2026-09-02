//! MGGA_XC_ZLP fxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_zlp.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_xc_zlp_fxc_pol(
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
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = M_CBRT3;
        let t4 = pow_1_3(1.0 / M_PI);
        let t5 = t2 * t4;
        let t6 = M_CBRT4;
        let t7 = t6 * t6;
        let t11 = sigma0 + 2.0 * sigma1 + sigma2;
        let t12 = rho0 + rho1;
        let t13 = t12 * t12;
        let t14 = pow_1_3(t12);
        let t15 = t14 * t14;
        let t17 = 1.0 / t15 / t13;
        let t19 = pow_1_3(rho0);
        let t20 = t19 * t19;
        let t22 = 1.0 / t20 / rho0;
        let t23 = lapl0 * t22;
        let t24 = rho0 - rho1;
        let t25 = 1.0 / t12;
        let t26 = t24 * t25;
        let t28 = 1.0 / 2.0 + t26 / 2.0;
        let t29 = pow_1_3(t28);
        let t30 = t29 * t29;
        let t31 = t30 * t28;
        let t33 = pow_1_3(rho1);
        let t34 = t33 * t33;
        let t36 = 1.0 / t34 / rho1;
        let t37 = lapl1 * t36;
        let t39 = 1.0 / 2.0 - t26 / 2.0;
        let t40 = pow_1_3(t39);
        let t41 = t40 * t40;
        let t42 = t41 * t39;
        let t49 = 0.207108 * t5 * t7 + 0.005387725 * t5 * t7 * (t11 * t17 / 8.0 - t23 * t31 / 8.0 - t37 * t42 / 8.0);
        let t52 = 1.0 + 488.4942506669168 / t14;
        let t53 = rmath::ln(t52);
        let t56 = 1.0 - 0.002047107 * t53 * t14;
        let t58 = t2 * t2;
        let t59 = t49 * t56 * t58;
        let t60 = 1.0 / t4;
        let t61 = t60 * t6;
        let t62 = t61 * t14;
        let t63 = t59 * t62;
        let tzk0 = -t63 / 3.0;
        zk[ip] += tzk0;
        let t65 = 4.0 / 9.0 * t63;
        let t66 = t14 * t12;
        let t67 = t13 * t12;
        let t69 = 1.0 / t15 / t67;
        let t71 = t11 * t69 / 3.0;
        let t72 = rho0 * rho0;
        let t74 = 1.0 / t20 / t72;
        let t75 = lapl0 * t74;
        let t78 = 1.0 / t13;
        let t79 = t24 * t78;
        let t81 = t25 / 2.0 - t79 / 2.0;
        let t82 = t30 * t81;
        let t85 = -t81;
        let t86 = t41 * t85;
        let t89 = -t71 + 5.0 / 24.0 * t75 * t31 - 5.0 / 24.0 * t23 * t82 - 5.0 / 24.0 * t37 * t86;
        let t90 = t66 * t89;
        let t93 = t66 * t49;
        let t94 = 1.0 / t52;
        let t97 = 1.0 / t15;
        let t100 = 0.3333333333333333 * t25 * t94 - 0.000682369 * t53 * t97;
        let t103 = t58 * t60 * t6;
        let t105 = t93 * t100 * t103 / 3.0;
        let tvrho0 = -t65 - 0.0215509 * t90 * t56 - t105;
        vrho[ip * 2] += tvrho0;
        let t107 = -t25 / 2.0 - t79 / 2.0;
        let t108 = t30 * t107;
        let t111 = rho1 * rho1;
        let t113 = 1.0 / t34 / t111;
        let t114 = lapl1 * t113;
        let t117 = -t107;
        let t118 = t41 * t117;
        let t121 = -t71 - 5.0 / 24.0 * t23 * t108 + 5.0 / 24.0 * t114 * t42 - 5.0 / 24.0 * t37 * t118;
        let t122 = t66 * t121;
        let tvrho1 = -t65 - 0.0215509 * t122 * t56 - t105;
        vrho[ip * 2 + 1] += tvrho1;
        let t125 = 1.0 / t66;
        let t126 = t125 * t56;
        let tvsigma0 = -0.0026938625 * t126;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = -0.005387725 * t126;
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = tvsigma0;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t129 = t66 * t22;
        let t130 = t31 * t56;
        let tvlapl0 = 0.0026938625 * t129 * t130;
        vlapl[ip * 2] += tvlapl0;
        let t132 = t66 * t36;
        let t133 = t42 * t56;
        let tvlapl1 = 0.0026938625 * t132 * t133;
        vlapl[ip * 2 + 1] += tvlapl1;
        let tvtau0 = 0.0;
        vtau[ip * 2] += tvtau0;
        let tvtau1 = 0.0;
        vtau[ip * 2 + 1] += tvtau1;
        let t135 = t89 * t56;
        let t136 = t135 * t14;
        let t139 = t49 * t100 * t58;
        let t141 = 8.0 / 9.0 * t139 * t62;
        let t142 = t61 * t97;
        let t144 = 4.0 / 27.0 * t59 * t142;
        let t145 = t13 * t13;
        let t147 = 1.0 / t15 / t145;
        let t149 = 11.0 / 9.0 * t11 * t147;
        let t152 = 1.0 / t20 / t72 / rho0;
        let t153 = lapl0 * t152;
        let t158 = 1.0 / t29;
        let t159 = t81 * t81;
        let t160 = t158 * t159;
        let t163 = 1.0 / t67;
        let t164 = t24 * t163;
        let t165 = -t78 + t164;
        let t166 = t30 * t165;
        let t169 = 1.0 / t40;
        let t170 = t85 * t85;
        let t171 = t169 * t170;
        let t174 = -t165;
        let t175 = t41 * t174;
        let t178 = t149 - 5.0 / 9.0 * t153 * t31 + 25.0 / 36.0 * t75 * t82 - 5.0 / 36.0 * t23 * t160 - 5.0 / 24.0 * t23 * t166 - 5.0 / 36.0 * t37 * t171 - 5.0 / 24.0 * t37 * t175;
        let t179 = t66 * t178;
        let t182 = t90 * t100;
        let t187 = 1.0 / t14 / t13;
        let t188 = t52 * t52;
        let t189 = 1.0 / t188;
        let t193 = 1.0 / t15 / t12;
        let t196 = -0.2222222222222222 * t78 * t94 + 54.277138962990755 * t187 * t189 + 0.00045491266666666667 * t53 * t193;
        let t199 = t93 * t196 * t103 / 3.0;
        let tv2rho20 = -0.057469066666666666 * t136 - t141 - t144 - 0.0215509 * t179 * t56 - 0.0431018 * t182 - t199;
        v2rho2[ip * 3] += tv2rho20;
        let t201 = t14 * t121;
        let t202 = t201 * t56;
        let t206 = t158 * t107;
        let t207 = t206 * t81;
        let t210 = t30 * t24;
        let t211 = t210 * t163;
        let t216 = t169 * t117;
        let t217 = t216 * t85;
        let t220 = t41 * t24;
        let t221 = t220 * t163;
        let t224 = t149 + 25.0 / 72.0 * t75 * t108 - 5.0 / 36.0 * t23 * t207 - 5.0 / 24.0 * t23 * t211 + 25.0 / 72.0 * t114 * t86 - 5.0 / 36.0 * t37 * t217 + 5.0 / 24.0 * t37 * t221;
        let t225 = t66 * t224;
        let t228 = t122 * t100;
        let tv2rho21 = -0.028734533333333333 * t136 - t141 - t144 - 0.028734533333333333 * t202 - 0.0215509 * t225 * t56 - 0.0215509 * t228 - 0.0215509 * t182 - t199;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t232 = t107 * t107;
        let t233 = t158 * t232;
        let t236 = t78 + t164;
        let t237 = t30 * t236;
        let t242 = 1.0 / t34 / t111 / rho1;
        let t243 = lapl1 * t242;
        let t248 = t117 * t117;
        let t249 = t169 * t248;
        let t252 = -t236;
        let t253 = t41 * t252;
        let t256 = t149 - 5.0 / 36.0 * t23 * t233 - 5.0 / 24.0 * t23 * t237 - 5.0 / 9.0 * t243 * t42 + 25.0 / 36.0 * t114 * t118 - 5.0 / 36.0 * t37 * t249 - 5.0 / 24.0 * t37 * t253;
        let t257 = t66 * t256;
        let tv2rho22 = -0.057469066666666666 * t202 - t141 - t144 - 0.0215509 * t257 * t56 - 0.0431018 * t228 - t199;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t261 = t187 * t56;
        let t263 = t125 * t100;
        let tv2rhosigma0 = 0.0035918166666666666 * t261 - 0.0026938625 * t263;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = 0.007183633333333333 * t261 - 0.005387725 * t263;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let tv2rhosigma2 = tv2rhosigma0;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let tv2rhosigma3 = tv2rhosigma2;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = tv2rhosigma1;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let tv2rhosigma5 = tv2rhosigma3;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t267 = t14 * t22;
        let t269 = 0.0035918166666666666 * t267 * t130;
        let t270 = t66 * t74;
        let t273 = t30 * t56;
        let t274 = t273 * t81;
        let t277 = t31 * t100;
        let t279 = 0.0026938625 * t129 * t277;
        let tv2rholapl0 = t269 - 0.0044897708333333335 * t270 * t130 + 0.0044897708333333335 * t129 * t274 + t279;
        v2rholapl[ip * 4] += tv2rholapl0;
        let t280 = t14 * t36;
        let t282 = 0.0035918166666666666 * t280 * t133;
        let t283 = t41 * t56;
        let t284 = t283 * t85;
        let t287 = t42 * t100;
        let t289 = 0.0026938625 * t132 * t287;
        let tv2rholapl1 = t282 + 0.0044897708333333335 * t132 * t284 + t289;
        v2rholapl[ip * 4 + 1] += tv2rholapl1;
        let t290 = t273 * t107;
        let tv2rholapl2 = t269 + 0.0044897708333333335 * t129 * t290 + t279;
        v2rholapl[ip * 4 + 2] += tv2rholapl2;
        let t293 = t66 * t113;
        let t296 = t283 * t117;
        let tv2rholapl3 = t282 - 0.0044897708333333335 * t293 * t133 + 0.0044897708333333335 * t132 * t296 + t289;
        v2rholapl[ip * 4 + 3] += tv2rholapl3;
        let tv2rhotau0 = 0.0;
        v2rhotau[ip * 4] += tv2rhotau0;
        let tv2rhotau1 = 0.0;
        v2rhotau[ip * 4 + 1] += tv2rhotau1;
        let tv2rhotau2 = 0.0;
        v2rhotau[ip * 4 + 2] += tv2rhotau2;
        let tv2rhotau3 = 0.0;
        v2rhotau[ip * 4 + 3] += tv2rhotau3;
        let tv2sigma20 = 0.0;
        v2sigma2[ip * 6] += tv2sigma20;
        let tv2sigma21 = 0.0;
        v2sigma2[ip * 6 + 1] += tv2sigma21;
        let tv2sigma22 = 0.0;
        v2sigma2[ip * 6 + 2] += tv2sigma22;
        let tv2sigma23 = 0.0;
        v2sigma2[ip * 6 + 3] += tv2sigma23;
        let tv2sigma24 = 0.0;
        v2sigma2[ip * 6 + 4] += tv2sigma24;
        let tv2sigma25 = 0.0;
        v2sigma2[ip * 6 + 5] += tv2sigma25;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip * 6] += tv2sigmalapl0;
        let tv2sigmalapl1 = 0.0;
        v2sigmalapl[ip * 6 + 1] += tv2sigmalapl1;
        let tv2sigmalapl2 = 0.0;
        v2sigmalapl[ip * 6 + 2] += tv2sigmalapl2;
        let tv2sigmalapl3 = 0.0;
        v2sigmalapl[ip * 6 + 3] += tv2sigmalapl3;
        let tv2sigmalapl4 = 0.0;
        v2sigmalapl[ip * 6 + 4] += tv2sigmalapl4;
        let tv2sigmalapl5 = 0.0;
        v2sigmalapl[ip * 6 + 5] += tv2sigmalapl5;
        let tv2sigmatau0 = 0.0;
        v2sigmatau[ip * 6] += tv2sigmatau0;
        let tv2sigmatau1 = 0.0;
        v2sigmatau[ip * 6 + 1] += tv2sigmatau1;
        let tv2sigmatau2 = 0.0;
        v2sigmatau[ip * 6 + 2] += tv2sigmatau2;
        let tv2sigmatau3 = 0.0;
        v2sigmatau[ip * 6 + 3] += tv2sigmatau3;
        let tv2sigmatau4 = 0.0;
        v2sigmatau[ip * 6 + 4] += tv2sigmatau4;
        let tv2sigmatau5 = 0.0;
        v2sigmatau[ip * 6 + 5] += tv2sigmatau5;
        let tv2lapl20 = 0.0;
        v2lapl2[ip * 3] += tv2lapl20;
        let tv2lapl21 = 0.0;
        v2lapl2[ip * 3 + 1] += tv2lapl21;
        let tv2lapl22 = 0.0;
        v2lapl2[ip * 3 + 2] += tv2lapl22;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip * 4] += tv2lapltau0;
        let tv2lapltau1 = 0.0;
        v2lapltau[ip * 4 + 1] += tv2lapltau1;
        let tv2lapltau2 = 0.0;
        v2lapltau[ip * 4 + 2] += tv2lapltau2;
        let tv2lapltau3 = 0.0;
        v2lapltau[ip * 4 + 3] += tv2lapltau3;
        let tv2tau20 = 0.0;
        v2tau2[ip * 3] += tv2tau20;
        let tv2tau21 = 0.0;
        v2tau2[ip * 3 + 1] += tv2tau21;
        let tv2tau22 = 0.0;
        v2tau2[ip * 3 + 2] += tv2tau22;
    }
}
