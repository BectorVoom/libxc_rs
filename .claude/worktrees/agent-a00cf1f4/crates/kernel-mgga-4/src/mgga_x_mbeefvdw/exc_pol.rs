//! MGGA_X_MBEEFVDW exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 133 shared lines across all orders.
//! Delta: 133 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_mbeefvdw_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
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
        // --- shared preamble (133 lines) ---
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
        let t20 = t19 + 1.0;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = pow_1_3(t7);
        let t28 = t26 * t27;
        let t29 = M_CBRT6;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t40 = sigma0 * t39;
        let t43 = 0.65124e1 + t34 * t40 / 24.0;
        let t44 = 1.0 / t43;
        let t46 = t34 * t40 * t44;
        let t48 = t46 / 12.0 - 1.0;
        let t49 = t48 * t48;
        let t51 = t49 * t48;
        let t54 = t49 * t49;
        let t57 = 1.0 / t37 / rho0;
        let t63 = 5.0 / 9.0 * (tau0 * t57 - t40 / 8.0) * t29 * t33;
        let t64 = 10000.0 <= t63;
        let t65 = 10000.0 < t63;
        let t66 = piecewise3(t65, t63, 10000.0);
        let t67 = t66 * t66;
        let t70 = t67 * t66;
        let t71 = 1.0 / t70;
        let t72 = t67 * t67;
        let t73 = 1.0 / t72;
        let t76 = piecewise3(t65, 10000.0, t63);
        let t77 = t76 * t76;
        let t78 = 1.0 - t77;
        let t79 = t78 * t78;
        let t80 = t79 * t78;
        let t81 = t77 * t76;
        let t82 = 1.0 + t81;
        let t84 = t81 * t82 + 1.0;
        let t85 = 1.0 / t84;
        let t87 = piecewise3(t64, 1.0 - 3.0 / t67 - t71 + 3.0 * t73, -t80 * t85);
        let t89 = t87 * t87;
        let t91 = t89 * t87;
        let t93 = t89 * t89;
        let t97 = 3.0 / 8.0 + 35.0 / 8.0 * t54 - 15.0 / 4.0 * t49;
        let t100 = 3.0 / 8.0 + 35.0 / 8.0 * t93 - 15.0 / 4.0 * t89;
        let t105 = 5.0 / 2.0 * t91 - 3.0 / 2.0 * t87;
        let t109 = -1.0 / 2.0 + 3.0 / 2.0 * t89;
        let t112 = t97 * t87;
        let t114 = -0.851282539125e-1 * t49 - 0.50282912e-1 * t51 + 0.1214700985e-1 * t46 + 0.618699843125e-2 * t54 - 0.6972770593e-1 * t87 + 0.217681859775e-1 * t89 + 0.351985355e-2 * t91 + 0.61919587625e-3 * t93 - 0.340722258e-8 * t97 * t100 + 0.574317889e-7 * t97 * t105 - 0.500749348e-6 * t97 * t109 + 0.919317034e-6 * t112;
        let t117 = 5.0 / 2.0 * t51 - t46 / 8.0 + 3.0 / 2.0;
        let t124 = t117 * t87;
        let t127 = -1.0 / 2.0 + 3.0 / 2.0 * t49;
        let t134 = t127 * t87;
        let t142 = t48 * t87;
        let t144 = 0.10451438955835e1 + 0.397324768e-8 * t117 * t100 - 0.549909413e-7 * t117 * t105 + 0.133707403e-6 * t117 * t109 + 0.192374554e-1 * t124 + 0.201895739e-6 * t127 * t100 - 0.657949254e-6 * t127 * t105 - 0.521818079e-2 * t127 * t109 - 0.222650139e-1 * t134 - 0.100478906e-6 * t48 * t100 - 0.608338264e-2 * t48 * t105 + 0.318024096e-1 * t48 * t109 + 0.453837246e-1 * t142;
        let t145 = t114 + t144;
        let t149 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t145);
        let t150 = rho1 <= dens_threshold;
        let t151 = -t17;
        let t153 = piecewise5(t15, t12, t11, t16, t151 * t8);
        let t154 = t153 + 1.0;
        let t155 = t154 <= zeta_threshold;
        let t156 = pow_1_3(t154);
        let t158 = piecewise3(t155, t23, t156 * t154);
        let t159 = t158 * t27;
        let t160 = rho1 * rho1;
        let t161 = pow_1_3(rho1);
        let t162 = t161 * t161;
        let t164 = 1.0 / t162 / t160;
        let t165 = sigma2 * t164;
        let t168 = 0.65124e1 + t34 * t165 / 24.0;
        let t169 = 1.0 / t168;
        let t171 = t34 * t165 * t169;
        let t174 = t171 / 12.0 - 1.0;
        let t175 = t174 * t174;
        let t176 = t175 * t175;
        let t179 = 1.0 / t162 / rho1;
        let t185 = 5.0 / 9.0 * (tau1 * t179 - t165 / 8.0) * t29 * t33;
        let t186 = 10000.0 <= t185;
        let t187 = 10000.0 < t185;
        let t188 = piecewise3(t187, t185, 10000.0);
        let t189 = t188 * t188;
        let t192 = t189 * t188;
        let t193 = 1.0 / t192;
        let t194 = t189 * t189;
        let t195 = 1.0 / t194;
        let t198 = piecewise3(t187, 10000.0, t185);
        let t199 = t198 * t198;
        let t200 = 1.0 - t199;
        let t201 = t200 * t200;
        let t202 = t201 * t200;
        let t203 = t199 * t198;
        let t204 = 1.0 + t203;
        let t206 = t203 * t204 + 1.0;
        let t207 = 1.0 / t206;
        let t209 = piecewise3(t186, 1.0 - 3.0 / t189 - t193 + 3.0 * t195, -t202 * t207);
        let t212 = t209 * t209;
        let t213 = t212 * t212;
        let t215 = t175 * t174;
        let t218 = t212 * t209;
        let t222 = 3.0 / 8.0 + 35.0 / 8.0 * t176 - 15.0 / 4.0 * t175;
        let t225 = 3.0 / 8.0 + 35.0 / 8.0 * t213 - 15.0 / 4.0 * t212;
        let t230 = 5.0 / 2.0 * t218 - 3.0 / 2.0 * t209;
        let t234 = -1.0 / 2.0 + 3.0 / 2.0 * t212;
        let t237 = 0.10451438955835e1 + 0.1214700985e-1 * t171 + 0.618699843125e-2 * t176 - 0.6972770593e-1 * t209 - 0.851282539125e-1 * t175 + 0.61919587625e-3 * t213 - 0.50282912e-1 * t215 + 0.217681859775e-1 * t212 + 0.351985355e-2 * t218 - 0.340722258e-8 * t222 * t225 + 0.574317889e-7 * t222 * t230 - 0.500749348e-6 * t222 * t234;
        let t238 = t222 * t209;
        let t240 = t174 * t209;
        let t249 = -1.0 / 2.0 + 3.0 / 2.0 * t175;
        let t250 = t249 * t209;
        let t260 = 5.0 / 2.0 * t215 - t171 / 8.0 + 3.0 / 2.0;
        let t261 = t260 * t209;
        let t269 = 0.919317034e-6 * t238 + 0.453837246e-1 * t240 + 0.318024096e-1 * t174 * t234 - 0.608338264e-2 * t174 * t230 - 0.100478906e-6 * t174 * t225 - 0.222650139e-1 * t250 - 0.521818079e-2 * t249 * t234 - 0.657949254e-6 * t249 * t230 + 0.201895739e-6 * t249 * t225 + 0.192374554e-1 * t261 + 0.133707403e-6 * t260 * t234 - 0.549909413e-7 * t260 * t230 + 0.397324768e-8 * t260 * t225;
        let t270 = t237 + t269;
        let t274 = piecewise3(t150, 0.0, -3.0 / 8.0 * t6 * t159 * t270);
        let tzk0 = t149 + t274;
        zk[ip] += tzk0;
    }
}
