//! GGA_X_OPTX kxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_optx.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_optx_kxc_unpol(
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
    param_gamma: f64,
    param_b: f64,
    param_a: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = param_gamma * param_gamma;
        let t21 = param_b * t20;
        let t22 = sigma[ip] * sigma[ip];
        let t23 = t21 * t22;
        let t24 = M_CBRT2;
        let t25 = rho[ip] * rho[ip];
        let t26 = t25 * t25;
        let t27 = t26 * rho[ip];
        let t32 = t24 * t24;
        let t33 = t18 * t18;
        let t35 = 1.0 / t33 / t25;
        let t38 = param_gamma * sigma[ip] * t32 * t35 + 1.0;
        let t39 = t38 * t38;
        let t40 = 1.0 / t39;
        let t41 = t24 / t18 / t27 * t40;
        let t44 = 2.0 * t23 * t41 + param_a;
        let t48 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t44);
        let tzk0 = 2.0 * t48;
        zk[ip] += tzk0;
        let t50 = t17 / t33;
        let t54 = t26 * t25;
        let t58 = t24 / t18 / t54 * t40;
        let t62 = param_b * t20 * param_gamma;
        let t63 = t22 * sigma[ip];
        let t64 = t26 * t26;
        let t65 = t64 * rho[ip];
        let t66 = 1.0 / t65;
        let t69 = 1.0 / t39 / t38;
        let t73 = -32.0 / 3.0 * t23 * t58 + 64.0 / 3.0 * t62 * t63 * t66 * t69;
        let t78 = piecewise3(t2, 0.0, -t6 * t50 * t44 / 8.0 - 3.0 / 8.0 * t6 * t19 * t73);
        let tvrho0 = 2.0 * rho[ip] * t78 + 2.0 * t48;
        vrho[ip] += tvrho0;
        let t81 = t21 * sigma[ip];
        let t84 = 1.0 / t64;
        let t89 = -8.0 * t62 * t22 * t84 * t69 + 4.0 * t81 * t41;
        let t93 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t89);
        let tvsigma0 = 2.0 * rho[ip] * t93;
        vsigma[ip] += tvsigma0;
        let t98 = t17 / t33 / rho[ip];
        let t105 = t25 * rho[ip];
        let t106 = t26 * t105;
        let t110 = t24 / t18 / t106 * t40;
        let t113 = t64 * t25;
        let t114 = 1.0 / t113;
        let t119 = t20 * t20;
        let t120 = param_b * t119;
        let t121 = t22 * t22;
        let t122 = t120 * t121;
        let t123 = t64 * t26;
        let t125 = 1.0 / t33 / t123;
        let t126 = t39 * t39;
        let t127 = 1.0 / t126;
        let t129 = t125 * t127 * t32;
        let t132 = 608.0 / 9.0 * t23 * t110 - 2752.0 / 9.0 * t62 * t63 * t114 * t69 + 512.0 / 3.0 * t122 * t129;
        let t137 = piecewise3(t2, 0.0, t6 * t98 * t44 / 12.0 - t6 * t50 * t73 / 4.0 - 3.0 / 8.0 * t6 * t19 * t132);
        let tv2rho20 = 2.0 * rho[ip] * t137 + 4.0 * t78;
        v2rho2[ip] += tv2rho20;
        let t149 = t120 * t63;
        let t150 = t64 * t105;
        let t152 = 1.0 / t33 / t150;
        let t154 = t152 * t127 * t32;
        let t157 = -64.0 / 3.0 * t81 * t58 + 320.0 / 3.0 * t62 * t22 * t66 * t69 - 64.0 * t149 * t154;
        let t162 = piecewise3(t2, 0.0, -t6 * t50 * t89 / 8.0 - 3.0 / 8.0 * t6 * t19 * t157);
        let tv2rhosigma0 = 2.0 * rho[ip] * t162 + 2.0 * t93;
        v2rhosigma[ip] += tv2rhosigma0;
        let t171 = t120 * t22;
        let t175 = 1.0 / t33 / t113 * t127 * t32;
        let t178 = -32.0 * t62 * sigma[ip] * t84 * t69 + 24.0 * t171 * t175 + 4.0 * t21 * t41;
        let t182 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t178);
        let tv2sigma20 = 2.0 * rho[ip] * t182;
        v2sigma2[ip] += tv2sigma20;
        let t185 = t17 * t35;
        let t198 = t24 / t18 / t64 * t40;
        let t201 = 1.0 / t150;
        let t206 = t64 * t27;
        let t210 = 1.0 / t33 / t206 * t127 * t32;
        let t214 = param_b * t119 * param_gamma;
        let t215 = t121 * sigma[ip];
        let t216 = t214 * t215;
        let t217 = t64 * t64;
        let t221 = 1.0 / t126 / t38;
        let t223 = 1.0 / t18 / t217 * t221 * t24;
        let t226 = -13376.0 / 27.0 * t23 * t198 + 102016.0 / 27.0 * t62 * t63 * t201 * t69 - 4608.0 * t122 * t210 + 32768.0 / 9.0 * t216 * t223;
        let t231 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t185 * t44 + t6 * t98 * t73 / 4.0 - 3.0 / 8.0 * t6 * t50 * t132 - 3.0 / 8.0 * t6 * t19 * t226);
        let tv3rho30 = 2.0 * rho[ip] * t231 + 6.0 * t137;
        v3rho3[ip] += tv3rho30;
        let t249 = t214 * t121;
        let t254 = 1.0 / t18 / t64 / t106 * t221 * t24;
        let t257 = 1216.0 / 9.0 * t81 * t110 - 10688.0 / 9.0 * t62 * t22 * t114 * t69 + 1600.0 * t149 * t129 - 4096.0 / 3.0 * t249 * t254;
        let t262 = piecewise3(t2, 0.0, t6 * t98 * t89 / 12.0 - t6 * t50 * t157 / 4.0 - 3.0 / 8.0 * t6 * t19 * t257);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t262 + 4.0 * t162;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t270 = t66 * t69;
        let t276 = t214 * t63;
        let t277 = t64 * t54;
        let t281 = 1.0 / t18 / t277 * t221 * t24;
        let t284 = -64.0 / 3.0 * t21 * t58 + 896.0 / 3.0 * t62 * t270 * sigma[ip] - 512.0 * t171 * t154 + 512.0 * t276 * t281;
        let t289 = piecewise3(t2, 0.0, -t6 * t50 * t178 / 8.0 - 3.0 / 8.0 * t6 * t19 * t284);
        let tv3rhosigma20 = 2.0 * rho[ip] * t289 + 2.0 * t182;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t298 = t214 * t22;
        let t302 = 1.0 / t18 / t206 * t221 * t24;
        let t305 = 144.0 * t120 * sigma[ip] * t175 - 48.0 * t62 * t84 * t69 - 192.0 * t298 * t302;
        let t309 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t305);
        let tv3sigma30 = 2.0 * rho[ip] * t309;
        v3sigma3[ip] += tv3sigma30;
    }
}
