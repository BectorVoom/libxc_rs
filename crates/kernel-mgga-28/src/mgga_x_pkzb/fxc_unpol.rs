//! MGGA_X_PKZB fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 44 shared lines across all orders.
//! Delta: 51 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_pkzb_fxc_unpol(
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
        // --- shared preamble (44 lines) ---
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t21 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t21 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t19 * t19;
        let t33 = 1.0 / t31 / t30;
        let t34 = t29 * t33;
        let t35 = t26 * t34;
        let t37 = tau[ip] * t28;
        let t39 = 1.0 / t31 / rho[ip];
        let t44 = t26 * t37 * t39 / 4.0 - 9.0 / 20.0 - t35 / 288.0;
        let t45 = t44 * t44;
        let t47 = t44 * t21;
        let t48 = t47 * t25;
        let t51 = t21 * t21;
        let t53 = 1.0 / t23 / t22;
        let t54 = t51 * t53;
        let t55 = sigma[ip] * sigma[ip];
        let t56 = t55 * t27;
        let t57 = t30 * t30;
        let t58 = t57 * rho[ip];
        let t60 = 1.0 / t19 / t58;
        let t64 = 0.804e0 + 5.0 / 972.0 * t35 + 146.0 / 2025.0 * t45 - 73.0 / 9720.0 * t48 * t34 + 0.45818468001825619316e-3 * t54 * t56 * t60;
        let t67 = 0.1804e1 - 0.646416e0 / t64;
        let t71 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t18 * t19 * t67);
        let tzk0 = 2.0 * t71;
        zk[ip] += tzk0;
        // --- vxc delta (29 lines) ---
        let t72 = 1.0 / t31;
        let t77 = t4 * t18;
        let t78 = t64 * t64;
        let t79 = 1.0 / t78;
        let t80 = t19 * t79;
        let t81 = t30 * rho[ip];
        let t83 = 1.0 / t31 / t81;
        let t84 = t29 * t83;
        let t85 = t26 * t84;
        let t91 = -5.0 / 12.0 * t26 * t37 * t33 + t85 / 108.0;
        let t94 = t91 * t21;
        let t95 = t94 * t25;
        let t100 = t57 * t30;
        let t102 = 1.0 / t19 / t100;
        let t106 = -10.0 / 729.0 * t85 + 292.0 / 2025.0 * t44 * t91 - 73.0 / 9720.0 * t95 * t34 + 73.0 / 3645.0 * t48 * t84 - 0.24436516267640330302e-2 * t54 * t56 * t102;
        let t111 = piecewise3(t3, 0.0, -t7 * t18 * t72 * t67 / 8.0 - 0.16551095363746320496e0 * t77 * t80 * t106);
        let tvrho0 = 2.0 * rho[ip] * t111 + 2.0 * t71;
        vrho[ip] += tvrho0;
        let t117 = t25 * t28;
        let t118 = t117 * t33;
        let t119 = t47 * t118;
        let t123 = t54 * t27 * t60 * sigma[ip];
        let t125 = 5.0 / 972.0 * t26 * t28 * t33 - 146.0 / 18225.0 * t119 + 0.96852413827153753492e-3 * t123;
        let t129 = piecewise3(t3, 0.0, -0.16551095363746320496e0 * t77 * t80 * t125);
        let tvsigma0 = 2.0 * rho[ip] * t129;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t131 = t117 * t39;
        let t140 = 73.0 / 2025.0 * t47 * t131 - 73.0 / 19440.0 * t54 * t27 / t19 / t57 * sigma[ip];
        let t144 = piecewise3(t3, 0.0, -0.16551095363746320496e0 * t77 * t80 * t140);
        let tvtau0 = 2.0 * rho[ip] * t144;
        vtau[ip] += tvtau0;
        // --- fxc delta (this level) (51 lines) ---
        let t151 = t72 * t79;
        let t156 = 1.0 / t78 / t64;
        let t157 = t19 * t156;
        let t158 = t106 * t106;
        let t163 = 1.0 / t31 / t57;
        let t164 = t29 * t163;
        let t165 = t26 * t164;
        let t167 = t91 * t91;
        let t173 = 10.0 / 9.0 * t26 * t37 * t83 - 11.0 / 324.0 * t165;
        let t176 = t173 * t21;
        let t177 = t176 * t25;
        let t184 = t57 * t81;
        let t186 = 1.0 / t19 / t184;
        let t190 = 110.0 / 2187.0 * t165 + 292.0 / 2025.0 * t167 + 292.0 / 2025.0 * t44 * t173 - 73.0 / 9720.0 * t177 * t34 + 146.0 / 3645.0 * t95 * t84 - 803.0 / 10935.0 * t48 * t164 + 0.15476460302838875858e-1 * t54 * t56 * t186;
        let t195 = piecewise3(t3, 0.0, t7 * t18 * t39 * t67 / 12.0 - 0.11034063575830880331e0 * t77 * t151 * t106 + 0.33102190727492640992e0 * t77 * t157 * t158 - 0.16551095363746320496e0 * t77 * t80 * t190);
        let tv2rho20 = 2.0 * rho[ip] * t195 + 4.0 * t111;
        v2rho2[ip] += tv2rho20;
        let t201 = t77 * t19;
        let t202 = t156 * t125;
        let t203 = t202 * t106;
        let t209 = t94 * t118;
        let t211 = t117 * t83;
        let t212 = t47 * t211;
        let t216 = t54 * t27 * t102 * sigma[ip];
        let t218 = -10.0 / 729.0 * t26 * t28 * t83 - 146.0 / 18225.0 * t209 + 1168.0 / 54675.0 * t212 - 0.51654620707815335196e-2 * t216;
        let t223 = piecewise3(t3, 0.0, -0.55170317879154401653e-1 * t77 * t151 * t125 + 0.33102190727492640992e0 * t201 * t203 - 0.16551095363746320496e0 * t77 * t80 * t218);
        let tv2rhosigma0 = 2.0 * rho[ip] * t223 + 2.0 * t129;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t229 = t156 * t140;
        let t230 = t229 * t106;
        let t237 = 73.0 / 2025.0 * t94 * t131 - 73.0 / 1215.0 * t119 + 949.0 / 58320.0 * t123;
        let t242 = piecewise3(t3, 0.0, -0.55170317879154401653e-1 * t77 * t151 * t140 + 0.33102190727492640992e0 * t201 * t230 - 0.16551095363746320496e0 * t77 * t80 * t237);
        let tv2rhotau0 = 2.0 * rho[ip] * t242 + 2.0 * t144;
        v2rhotau[ip] += tv2rhotau0;
        let t245 = t125 * t125;
        let t249 = 1.0 / t58;
        let t252 = t53 * t27;
        let t253 = t79 * t51 * t252;
        let t254 = t77 * t249 * t253;
        let t257 = piecewise3(t3, 0.0, 0.33102190727492640992e0 * t77 * t157 * t245 - 0.16950901996748250202e-3 * t254);
        let tv2sigma20 = 2.0 * rho[ip] * t257;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t259 = t229 * t125;
        let t262 = 1.0 / t57;
        let t264 = t77 * t262 * t253;
        let t267 = piecewise3(t3, 0.0, 0.33102190727492640992e0 * t201 * t259 + 0.66295196793057964127e-3 * t264);
        let tv2sigmatau0 = 2.0 * rho[ip] * t267;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let t269 = t140 * t140;
        let t273 = 1.0 / t81;
        let t278 = piecewise3(t3, 0.0, 0.33102190727492640992e0 * t77 * t157 * t269 - 0.29832838556876083857e-2 * t77 * t273 * t253);
        let tv2tau20 = 2.0 * rho[ip] * t278;
        v2tau2[ip] += tv2tau20;
    }
}
