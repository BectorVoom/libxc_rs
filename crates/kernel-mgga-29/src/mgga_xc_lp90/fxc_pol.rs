//! MGGA_XC_LP90 fxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_lp90.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_xc_lp90_fxc_pol(
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
        let t3 = sigma0 + 2.0 * sigma1 + sigma2;
        let t4 = rho0 + rho1;
        let t5 = t4 * t4;
        let t6 = pow_1_3(t4);
        let t7 = t6 * t6;
        let t9 = 1.0 / t7 / t5;
        let t12 = pow_1_3(rho0);
        let t13 = t12 * t12;
        let t15 = 1.0 / t13 / rho0;
        let t16 = lapl0 * t15;
        let t17 = rho0 - rho1;
        let t18 = 1.0 / t4;
        let t19 = t17 * t18;
        let t21 = 1.0 / 2.0 + t19 / 2.0;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t24 = t23 * t21;
        let t27 = pow_1_3(rho1);
        let t28 = t27 * t27;
        let t30 = 1.0 / t28 / rho1;
        let t31 = lapl1 * t30;
        let t33 = 1.0 / 2.0 - t19 / 2.0;
        let t34 = pow_1_3(t33);
        let t35 = t34 * t34;
        let t36 = t35 * t33;
        let t39 = 0.80569e0 + 0.37655e-3 * t3 * t9 - 0.37655e-3 * t16 * t24 - 0.37655e-3 * t31 * t36;
        let t40 = 1.0 / t6;
        let t41 = t40 + 0.40743e-2;
        let t42 = 1.0 / t41;
        let tzk0 = -t39 * t42;
        zk[ip] += tzk0;
        let t44 = t5 * t4;
        let t46 = 1.0 / t7 / t44;
        let t48 = 0.10041333333333333333e-2 * t3 * t46;
        let t49 = rho0 * rho0;
        let t51 = 1.0 / t13 / t49;
        let t52 = lapl0 * t51;
        let t55 = 1.0 / t5;
        let t56 = t17 * t55;
        let t58 = t18 / 2.0 - t56 / 2.0;
        let t59 = t23 * t58;
        let t62 = -t58;
        let t63 = t35 * t62;
        let t66 = -t48 + 0.62758333333333333333e-3 * t52 * t24 - 0.62758333333333333333e-3 * t16 * t59 - 0.62758333333333333333e-3 * t31 * t63;
        let t70 = t41 * t41;
        let t71 = 1.0 / t70;
        let t73 = t40 * t39 * t71 / 3.0;
        let tvrho0 = -t4 * t66 * t42 - t73 + tzk0;
        vrho[ip * 2] += tvrho0;
        let t75 = -t18 / 2.0 - t56 / 2.0;
        let t76 = t23 * t75;
        let t79 = rho1 * rho1;
        let t81 = 1.0 / t28 / t79;
        let t82 = lapl1 * t81;
        let t85 = -t75;
        let t86 = t35 * t85;
        let t89 = -t48 - 0.62758333333333333333e-3 * t16 * t76 + 0.62758333333333333333e-3 * t82 * t36 - 0.62758333333333333333e-3 * t31 * t86;
        let tvrho1 = -t4 * t89 * t42 - t73 + tzk0;
        vrho[ip * 2 + 1] += tvrho1;
        let t93 = 1.0 / t7 / t4;
        let t94 = t93 * t42;
        let tvsigma0 = -0.37655e-3 * t94;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = -0.7531e-3 * t94;
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = tvsigma0;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t97 = t4 * t15;
        let t98 = t24 * t42;
        let tvlapl0 = 0.37655e-3 * t97 * t98;
        vlapl[ip * 2] += tvlapl0;
        let t100 = t4 * t30;
        let t101 = t36 * t42;
        let tvlapl1 = 0.37655e-3 * t100 * t101;
        vlapl[ip * 2 + 1] += tvlapl1;
        let tvtau0 = 0.0;
        vtau[ip * 2] += tvtau0;
        let tvtau1 = 0.0;
        vtau[ip * 2 + 1] += tvtau1;
        let t103 = t66 * t42;
        let t105 = t39 * t71;
        let t107 = 1.0 / t6 / t4;
        let t109 = 2.0 / 9.0 * t105 * t107;
        let t110 = t5 * t5;
        let t112 = 1.0 / t7 / t110;
        let t114 = 0.36818222222222222221e-2 * t3 * t112;
        let t117 = 1.0 / t13 / t49 / rho0;
        let t118 = lapl0 * t117;
        let t123 = 1.0 / t22;
        let t124 = t58 * t58;
        let t125 = t123 * t124;
        let t128 = 1.0 / t44;
        let t129 = t17 * t128;
        let t130 = -t55 + t129;
        let t131 = t23 * t130;
        let t134 = 1.0 / t34;
        let t135 = t62 * t62;
        let t136 = t134 * t135;
        let t139 = -t130;
        let t140 = t35 * t139;
        let t143 = t114 - 0.16735555555555555555e-2 * t118 * t24 + 0.20919444444444444444e-2 * t52 * t59 - 0.41838888888888888889e-3 * t16 * t125 - 0.62758333333333333333e-3 * t16 * t131 - 0.41838888888888888889e-3 * t31 * t136 - 0.62758333333333333333e-3 * t31 * t140;
        let t147 = t40 * t66 * t71;
        let t151 = 1.0 / t70 / t41;
        let t153 = 2.0 / 9.0 * t93 * t39 * t151;
        let tv2rho20 = -2.0 * t103 - t109 - t4 * t143 * t42 - 2.0 / 3.0 * t147 - t153;
        v2rho2[ip * 3] += tv2rho20;
        let t154 = t89 * t42;
        let t157 = t123 * t75;
        let t158 = t157 * t58;
        let t161 = t23 * t17;
        let t162 = t161 * t128;
        let t167 = t134 * t85;
        let t168 = t167 * t62;
        let t171 = t35 * t17;
        let t172 = t171 * t128;
        let t175 = t114 + 0.10459722222222222222e-2 * t52 * t76 - 0.41838888888888888889e-3 * t16 * t158 - 0.62758333333333333333e-3 * t16 * t162 + 0.10459722222222222222e-2 * t82 * t63 - 0.41838888888888888889e-3 * t31 * t168 + 0.62758333333333333333e-3 * t31 * t172;
        let t179 = t40 * t89 * t71;
        let tv2rho21 = -t103 - t109 - t154 - t4 * t175 * t42 - t179 / 3.0 - t147 / 3.0 - t153;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t183 = t75 * t75;
        let t184 = t123 * t183;
        let t187 = t55 + t129;
        let t188 = t23 * t187;
        let t193 = 1.0 / t28 / t79 / rho1;
        let t194 = lapl1 * t193;
        let t199 = t85 * t85;
        let t200 = t134 * t199;
        let t203 = -t187;
        let t204 = t35 * t203;
        let t207 = t114 - 0.41838888888888888889e-3 * t16 * t184 - 0.62758333333333333333e-3 * t16 * t188 - 0.16735555555555555555e-2 * t194 * t36 + 0.20919444444444444444e-2 * t82 * t86 - 0.41838888888888888889e-3 * t31 * t200 - 0.62758333333333333333e-3 * t31 * t204;
        let tv2rho22 = -2.0 * t154 - t109 - t4 * t207 * t42 - 2.0 / 3.0 * t179 - t153;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t211 = t9 * t42;
        let t213 = t128 * t71;
        let tv2rhosigma0 = 0.62758333333333333333e-3 * t211 - 0.12551666666666666667e-3 * t213;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = 0.12551666666666666667e-2 * t211 - 0.25103333333333333333e-3 * t213;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let tv2rhosigma2 = tv2rhosigma0;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let tv2rhosigma3 = tv2rhosigma2;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = tv2rhosigma1;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let tv2rhosigma5 = tv2rhosigma3;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t217 = t15 * t24;
        let t219 = 0.37655e-3 * t217 * t42;
        let t220 = t4 * t51;
        let t223 = t23 * t42;
        let t224 = t223 * t58;
        let t227 = t40 * t15;
        let t228 = t24 * t71;
        let t230 = 0.12551666666666666667e-3 * t227 * t228;
        let tv2rholapl0 = t219 - 0.62758333333333333333e-3 * t220 * t98 + 0.62758333333333333333e-3 * t97 * t224 + t230;
        v2rholapl[ip * 4] += tv2rholapl0;
        let t231 = t30 * t36;
        let t233 = 0.37655e-3 * t231 * t42;
        let t234 = t35 * t42;
        let t235 = t234 * t62;
        let t238 = t40 * t30;
        let t239 = t36 * t71;
        let t241 = 0.12551666666666666667e-3 * t238 * t239;
        let tv2rholapl1 = t233 + 0.62758333333333333333e-3 * t100 * t235 + t241;
        v2rholapl[ip * 4 + 1] += tv2rholapl1;
        let t242 = t223 * t75;
        let tv2rholapl2 = t219 + 0.62758333333333333333e-3 * t97 * t242 + t230;
        v2rholapl[ip * 4 + 2] += tv2rholapl2;
        let t245 = t4 * t81;
        let t248 = t234 * t85;
        let tv2rholapl3 = t233 - 0.62758333333333333333e-3 * t245 * t101 + 0.62758333333333333333e-3 * t100 * t248 + t241;
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
