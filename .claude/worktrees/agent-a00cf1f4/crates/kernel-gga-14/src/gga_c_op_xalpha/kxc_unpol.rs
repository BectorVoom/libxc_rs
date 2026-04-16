//! GGA_C_OP_XALPHA kxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_xalpha.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_op_xalpha_kxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = 1.0 <= zeta_threshold;
        let t4 = t1 || rho[ip] / 2.0 <= dens_threshold;
        let t5 = zeta_threshold - 1.0;
        let t6 = -t5;
        let t7 = piecewise5(t1, t5, t1, t6, 0.0);
        let t8 = t7 * t7;
        let t9 = 1.0 - t8;
        let t10 = t9 * rho[ip];
        let t11 = 1.0 + t7;
        let t14 = t11 * rho[ip] / 2.0 <= dens_threshold;
        let t15 = M_CBRT3;
        let t16 = t15 * t15;
        let t18 = pow_1_3(1.0 / M_PI);
        let t20 = t16 / t18;
        let t21 = M_CBRT4;
        let t22 = M_CBRT2;
        let t23 = t21 * t22;
        let t24 = t11 <= zeta_threshold;
        let t25 = 1.0 - t7;
        let t26 = t25 <= zeta_threshold;
        let t27 = piecewise5(t24, t5, t26, t6, t7);
        let t28 = 1.0 + t27;
        let t29 = t28 * rho[ip];
        let t30 = pow_1_3(t29);
        let t35 = piecewise3(t14, 0.0, t20 * t23 / t30 / 9.0);
        let t39 = t25 * rho[ip] / 2.0 <= dens_threshold;
        let t40 = piecewise5(t26, t5, t24, t6, -t7);
        let t41 = 1.0 + t40;
        let t42 = t41 * rho[ip];
        let t43 = pow_1_3(t42);
        let t48 = piecewise3(t39, 0.0, t20 * t23 / t43 / 9.0);
        let t49 = t35 + t48;
        let t50 = t49 == 0.0;
        let t51 = piecewise3(t50, f64::EPSILON, t49);
        let t54 = 0.390299956e1 / t51 + 0.5764e0;
        let t55 = t51 * t51;
        let t56 = t55 * t55;
        let t57 = 1.0 / t56;
        let t59 = t55 * t51;
        let t60 = 1.0 / t59;
        let t62 = 1.0 / t55;
        let t64 = 0.433132090567376656e2 * t57 + 0.190514637481962976e2 * t60 + 0.2094820520028e1 * t62;
        let t65 = 1.0 / t64;
        let tzk0 = piecewise3(t4, 0.0, -0.25e0 * t10 * t54 * t65);
        zk[ip] += tzk0;
        let t69 = t9 * t54;
        let t72 = t20 * t21;
        let t79 = piecewise3(t14, 0.0, -t72 * t22 / t30 / t29 * t28 / 27.0);
        let t86 = piecewise3(t39, 0.0, -t72 * t22 / t43 / t42 * t41 / 27.0);
        let t88 = piecewise3(t50, 0.0, t79 + t86);
        let t93 = t64 * t64;
        let t94 = 1.0 / t93;
        let t95 = t54 * t94;
        let t97 = 1.0 / t56 / t51;
        let t98 = t97 * t88;
        let t100 = t57 * t88;
        let t104 = -0.1732528362269506624e3 * t98 - 0.571543912445888928e2 * t100 - 0.4189641040056e1 * t60 * t88;
        let t109 = piecewise3(t4, 0.0, -0.25e0 * t69 * t65 + 0.97574989e0 * t10 * t62 * t88 * t65 + 0.25e0 * t10 * t95 * t104);
        let tvrho0 = rho[ip] * t109 + tzk0;
        vrho[ip] += tvrho0;
        let tvsigma0 = 0.0;
        vsigma[ip] += tvsigma0;
        let t112 = t9 * t62;
        let t113 = t88 * t65;
        let t119 = t88 * t88;
        let t124 = t28 * t28;
        let t125 = rho[ip] * rho[ip];
        let t133 = piecewise3(t14, 0.0, 4.0 / 81.0 * t72 * t22 / t30 / t125);
        let t134 = t41 * t41;
        let t142 = piecewise3(t39, 0.0, 4.0 / 81.0 * t72 * t22 / t43 / t125);
        let t144 = piecewise3(t50, 0.0, t133 + t142);
        let t149 = t10 * t62;
        let t150 = t88 * t94;
        let t151 = t150 * t104;
        let t155 = 1.0 / t93 / t64;
        let t156 = t54 * t155;
        let t157 = t104 * t104;
        let t162 = 1.0 / t56 / t55;
        let t163 = t162 * t119;
        let t167 = t97 * t119;
        let t175 = 0.866264181134753312e3 * t163 - 0.1732528362269506624e3 * t97 * t144 + 0.2286175649783555712e3 * t167 - 0.571543912445888928e2 * t57 * t144 + 0.12568923120168e2 * t57 * t119 - 0.4189641040056e1 * t60 * t144;
        let t180 = piecewise3(t4, 0.0, 0.195149978e1 * t112 * t113 + 0.5e0 * t69 * t94 * t104 - 0.195149978e1 * t10 * t60 * t119 * t65 + 0.97574989e0 * t10 * t62 * t144 * t65 - 0.195149978e1 * t149 * t151 - 0.5e0 * t10 * t156 * t157 + 0.25e0 * t10 * t95 * t175);
        let tv2rho20 = rho[ip] * t180 + 2.0 * t109;
        v2rho2[ip] += tv2rho20;
        let tv2rhosigma0 = 0.0;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2sigma20 = 0.0;
        v2sigma2[ip] += tv2sigma20;
        let t183 = t9 * t60;
        let t184 = t119 * t65;
        let t192 = t124 * t28;
        let t193 = t125 * rho[ip];
        let t201 = piecewise3(t14, 0.0, -28.0 / 243.0 * t72 * t22 / t30 / t193);
        let t202 = t134 * t41;
        let t210 = piecewise3(t39, 0.0, -28.0 / 243.0 * t72 * t22 / t43 / t193);
        let t212 = piecewise3(t50, 0.0, t201 + t210);
        let t218 = 1.0 / t56 / t59;
        let t219 = t119 * t88;
        let t222 = t162 * t88;
        let t239 = -0.5197585086808519872e4 * t218 * t219 + 0.2598792543404259936e4 * t222 * t144 - 0.1732528362269506624e3 * t97 * t212 - 0.1143087824891777856e4 * t162 * t219 + 0.6858526949350667136e3 * t98 * t144 - 0.571543912445888928e2 * t57 * t212 - 0.50275692480672e2 * t97 * t219 + 0.37706769360504e2 * t100 * t144 - 0.4189641040056e1 * t60 * t212;
        let t247 = t10 * t60;
        let t248 = t113 * t144;
        let t251 = t119 * t94;
        let t252 = t251 * t104;
        let t255 = t144 * t94;
        let t256 = t255 * t104;
        let t259 = t150 * t175;
        let t262 = t93 * t93;
        let t263 = 1.0 / t262;
        let t265 = t157 * t104;
        let t269 = t10 * t54;
        let t271 = t155 * t104 * t175;
        let t280 = t88 * t155;
        let t281 = t280 * t157;
        let t284 = -0.585449934e1 * t183 * t184 - 0.585449934e1 * t112 * t151 - 0.15e1 * t69 * t155 * t157 + 0.97574989e0 * t10 * t62 * t212 * t65 + 0.25e0 * t10 * t95 * t239 + 0.585449934e1 * t10 * t57 * t219 * t65 - 0.585449934e1 * t247 * t248 + 0.585449934e1 * t247 * t252 - 0.292724967e1 * t149 * t256 - 0.292724967e1 * t149 * t259 + 0.15e1 * t10 * t54 * t263 * t265 - 0.15e1 * t269 * t271 + 0.292724967e1 * t112 * t144 * t65 + 0.75e0 * t69 * t94 * t175 + 0.585449934e1 * t149 * t281;
        let t285 = piecewise3(t4, 0.0, t284);
        let tv3rho30 = rho[ip] * t285 + 3.0 * t180;
        v3rho3[ip] += tv3rho30;
        let tv3rho2sigma0 = 0.0;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let tv3rhosigma20 = 0.0;
        v3rhosigma2[ip] += tv3rhosigma20;
        let tv3sigma30 = 0.0;
        v3sigma3[ip] += tv3sigma30;
    }
}
