//! GGA_X_DK87 vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 70 shared lines across all orders.
//! Delta: 70 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_dk87_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_a1: f64,
    param_alpha: f64,
    param_b1: f64,
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
        // --- shared preamble (70 lines) ---
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = 1.0 / M_PI;
        let t29 = M_CBRT6;
        let t30 = t29 * t29;
        let t31 = t28 * t30;
        let t32 = M_PI * M_PI;
        let t33 = pow_1_3(t32);
        let t34 = 1.0 / t33;
        let t35 = t2 * t2;
        let t36 = t34 * t35;
        let t37 = pow_1_3(t28);
        let t38 = 1.0 / t37;
        let t40 = t31 * t36 * t38;
        let t41 = M_CBRT4;
        let t42 = t41 * sigma0;
        let t43 = rho0 * rho0;
        let t44 = pow_1_3(rho0);
        let t45 = t44 * t44;
        let t47 = 1.0 / t45 / t43;
        let t48 = f64::sqrt(sigma0);
        let t52 = f64::powf(t48 / t44 / rho0, param_alpha);
        let t54 = param_a1 * t52 + 1.0;
        let t56 = param_b1 * sigma0;
        let t58 = t56 * t47 + 1.0;
        let t59 = 1.0 / t58;
        let t60 = t47 * t54 * t59;
        let t64 = 1.0 + 7.0 / 11664.0 * t40 * t42 * t60;
        let t68 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t64);
        let t69 = rho1 <= dens_threshold;
        let t70 = -t16;
        let t72 = piecewise5(t14, t11, t10, t15, t70 * t7);
        let t73 = 1.0 + t72;
        let t74 = t73 <= zeta_threshold;
        let t75 = pow_1_3(t73);
        let t77 = piecewise3(t74, t22, t75 * t73);
        let t78 = t77 * t26;
        let t79 = t41 * sigma2;
        let t80 = rho1 * rho1;
        let t81 = pow_1_3(rho1);
        let t82 = t81 * t81;
        let t84 = 1.0 / t82 / t80;
        let t85 = f64::sqrt(sigma2);
        let t89 = f64::powf(t85 / t81 / rho1, param_alpha);
        let t91 = param_a1 * t89 + 1.0;
        let t93 = param_b1 * sigma2;
        let t95 = t93 * t84 + 1.0;
        let t96 = 1.0 / t95;
        let t97 = t84 * t91 * t96;
        let t101 = 1.0 + 7.0 / 11664.0 * t40 * t79 * t97;
        let t105 = piecewise3(t69, 0.0, -3.0 / 8.0 * t5 * t78 * t101);
        let tzk0 = t68 + t105;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (70 lines) ---
        let t106 = t6 * t6;
        let t107 = 1.0 / t106;
        let t108 = t16 * t107;
        let t110 = piecewise5(t10, 0.0, t14, 0.0, t7 - t108);
        let t113 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t110);
        let t114 = t113 * t26;
        let t118 = t26 * t26;
        let t119 = 1.0 / t118;
        let t120 = t25 * t119;
        let t123 = t5 * t120 * t64 / 8.0;
        let t124 = t43 * rho0;
        let t126 = 1.0 / t45 / t124;
        let t128 = t126 * t54 * t59;
        let t135 = t31 * t34 * t35 * t38 * t41;
        let t138 = t52 * param_alpha;
        let t139 = t138 * t59;
        let t143 = sigma0 * sigma0;
        let t144 = t41 * t143;
        let t145 = t43 * t43;
        let t146 = t145 * t43;
        let t148 = 1.0 / t44 / t146;
        let t150 = t58 * t58;
        let t151 = 1.0 / t150;
        let t153 = t54 * t151 * param_b1;
        let t157 = -7.0 / 4374.0 * t40 * t42 * t128 - 7.0 / 8748.0 * t135 * sigma0 * t126 * param_a1 * t139 + 7.0 / 4374.0 * t40 * t144 * t148 * t153;
        let t162 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t114 * t64 - t123 - 3.0 / 8.0 * t5 * t27 * t157);
        let t163 = t70 * t107;
        let t165 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t163);
        let t168 = piecewise3(t74, 0.0, 4.0 / 3.0 * t75 * t165);
        let t169 = t168 * t26;
        let t173 = t77 * t119;
        let t176 = t5 * t173 * t101 / 8.0;
        let t178 = piecewise3(t69, 0.0, -3.0 / 8.0 * t5 * t169 * t101 - t176);
        let tvrho0 = t68 + t105 + t6 * (t162 + t178);
        vrho[ip * 2] += tvrho0;
        let t182 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t108);
        let t185 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t182);
        let t186 = t185 * t26;
        let t191 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t186 * t64 - t123);
        let t193 = piecewise5(t14, 0.0, t10, 0.0, t7 - t163);
        let t196 = piecewise3(t74, 0.0, 4.0 / 3.0 * t75 * t193);
        let t197 = t196 * t26;
        let t201 = t80 * rho1;
        let t203 = 1.0 / t82 / t201;
        let t205 = t203 * t91 * t96;
        let t211 = t89 * param_alpha;
        let t212 = t211 * t96;
        let t216 = sigma2 * sigma2;
        let t217 = t41 * t216;
        let t218 = t80 * t80;
        let t219 = t218 * t80;
        let t221 = 1.0 / t81 / t219;
        let t223 = t95 * t95;
        let t224 = 1.0 / t223;
        let t226 = t91 * t224 * param_b1;
        let t230 = -7.0 / 4374.0 * t40 * t79 * t205 - 7.0 / 8748.0 * t135 * sigma2 * t203 * param_a1 * t212 + 7.0 / 4374.0 * t40 * t217 * t221 * t226;
        let t235 = piecewise3(t69, 0.0, -3.0 / 8.0 * t5 * t197 * t101 - t176 - 3.0 / 8.0 * t5 * t78 * t230);
        let tvrho1 = t68 + t105 + t6 * (t191 + t235);
        vrho[ip * 2 + 1] += tvrho1;
        let t238 = t31 * t36;
        let t239 = t38 * t41;
        let t248 = t145 * rho0;
        let t250 = 1.0 / t44 / t248;
        let t255 = 7.0 / 11664.0 * t238 * t239 * t60 + 7.0 / 23328.0 * t40 * t41 * t47 * param_a1 * t139 - 7.0 / 11664.0 * t40 * t42 * t250 * t153;
        let t259 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t255);
        let tvsigma0 = t6 * t259;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t268 = t218 * rho1;
        let t270 = 1.0 / t81 / t268;
        let t275 = 7.0 / 11664.0 * t238 * t239 * t97 + 7.0 / 23328.0 * t40 * t41 * t84 * param_a1 * t212 - 7.0 / 11664.0 * t40 * t79 * t270 * t226;
        let t279 = piecewise3(t69, 0.0, -3.0 / 8.0 * t5 * t78 * t275);
        let tvsigma2 = t6 * t279;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
