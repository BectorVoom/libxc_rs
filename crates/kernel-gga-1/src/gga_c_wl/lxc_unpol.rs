//! GGA_C_WL lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_wl.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_wl_lxc_unpol(
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
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = f64::sqrt(sigma[ip]);
        let t2 = pow_1_3(rho[ip]);
        let t4 = 1.0 / t2 / rho[ip];
        let t5 = t1 * t4;
        let t7 = -0.7486e0 + 0.6001e-1 * t5;
        let t8 = M_CBRT2;
        let t9 = t1 * t8;
        let t12 = M_CBRT3;
        let t14 = pow_1_3(1.0 / M_PI);
        let t15 = t12 * t14;
        let t16 = M_CBRT4;
        let t17 = t16 * t16;
        let t18 = 1.0 / t2;
        let t22 = 0.360073e1 + 0.18e1 * t9 * t4 + t15 * t17 * t18 / 4.0;
        let t23 = 1.0 / t22;
        let tzk0 = t7 * t23;
        zk[ip] += tzk0;
        let t26 = rho[ip] * t7;
        let t27 = t22 * t22;
        let t28 = 1.0 / t27;
        let t29 = rho[ip] * rho[ip];
        let t31 = 1.0 / t2 / t29;
        let t37 = -0.24e1 * t9 * t31 - t15 * t17 * t4 / 12.0;
        let t38 = t28 * t37;
        let tvrho0 = tzk0 - 0.80013333333333333333e-1 * t5 * t23 - t26 * t38;
        vrho[ip] += tvrho0;
        let t40 = 1.0 / t1;
        let t41 = t18 * t40;
        let t44 = t18 * t7;
        let t46 = t28 * t40 * t8;
        let tvsigma0 = 0.30005e-1 * t41 * t23 - 0.9e0 * t44 * t46;
        vsigma[ip] += tvsigma0;
        let t49 = t1 * t31;
        let t52 = t7 * t28;
        let t58 = 1.0 / t27 / t22;
        let t59 = t37 * t37;
        let t60 = t58 * t59;
        let t63 = t29 * rho[ip];
        let t65 = 1.0 / t2 / t63;
        let t71 = 0.56e1 * t9 * t65 + t15 * t17 * t31 / 9.0;
        let t72 = t28 * t71;
        let tv2rho20 = 0.26671111111111111107e-1 * t49 * t23 - 2.0 * t52 * t37 + 0.16002666666666666667e0 * t5 * t38 + 2.0 * t26 * t60 - t26 * t72;
        v2rho2[ip] += tv2rho20;
        let t74 = t4 * t40;
        let t79 = t4 * t7;
        let t82 = t2 * t2;
        let t84 = 1.0 / t82 / t29;
        let t88 = t44 * t58;
        let t89 = t40 * t8;
        let t90 = t89 * t37;
        let tv2rhosigma0 = -0.10001666666666666667e-1 * t74 * t23 - 0.30005e-1 * t41 * t38 + 0.3e0 * t79 * t46 + 0.72012e-1 * t84 * t28 * t8 + 0.18e1 * t88 * t90;
        v2rhosigma[ip] += tv2rhosigma0;
        let t94 = 1.0 / t1 / sigma[ip];
        let t95 = t18 * t94;
        let t99 = 1.0 / t82 / rho[ip];
        let t100 = 1.0 / sigma[ip];
        let t101 = t99 * t100;
        let t102 = t28 * t8;
        let t105 = t99 * t7;
        let t107 = t8 * t8;
        let t108 = t58 * t100 * t107;
        let t112 = t28 * t94 * t8;
        let tv2sigma20 = -0.150025e-1 * t95 * t23 - 0.54009e-1 * t101 * t102 + 0.162e1 * t105 * t108 + 0.45e0 * t44 * t112;
        v2sigma2[ip] += tv2sigma20;
        let t115 = t1 * t65;
        let t120 = t7 * t58;
        let t129 = t27 * t27;
        let t130 = 1.0 / t129;
        let t131 = t59 * t37;
        let t132 = t130 * t131;
        let t135 = t58 * t37;
        let t136 = t135 * t71;
        let t139 = t29 * t29;
        let t141 = 1.0 / t2 / t139;
        let t147 = -0.18666666666666666667e2 * t9 * t141 - 7.0 / 27.0 * t15 * t17 * t65;
        let t148 = t28 * t147;
        let tv3rho30 = -0.62232592592592592583e-1 * t115 * t23 - 0.8001333333333333333e-1 * t49 * t38 + 6.0 * t120 * t59 - 3.0 * t52 * t71 - 0.48008000000000000001e0 * t5 * t60 + 0.24004e0 * t5 * t72 - 6.0 * t26 * t132 + 6.0 * t26 * t136 - t26 * t148;
        v3rho3[ip] += tv3rho30;
        let t150 = t31 * t40;
        let t159 = t31 * t7;
        let t163 = 1.0 / t82 / t63;
        let t167 = t79 * t58;
        let t170 = t84 * t58;
        let t171 = t8 * t37;
        let t174 = t44 * t130;
        let t175 = t89 * t59;
        let t178 = t89 * t71;
        let tv3rho2sigma0 = 0.13335555555555555556e-1 * t150 * t23 + 0.20003333333333333334e-1 * t74 * t38 + 0.6001e-1 * t41 * t60 - 0.30005e-1 * t41 * t72 - 0.4e0 * t159 * t46 - 0.216036e0 * t163 * t28 * t8 - 0.12e1 * t167 * t90 - 0.288048e0 * t170 * t171 - 0.54e1 * t174 * t175 + 0.18e1 * t88 * t178;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t181 = t4 * t94;
        let t186 = t84 * t100;
        let t189 = t58 * t8;
        let t190 = t189 * t37;
        let t193 = t84 * t7;
        let t196 = 1.0 / t139;
        let t197 = t196 * t40;
        let t198 = t58 * t107;
        let t201 = t105 * t130;
        let t202 = t100 * t107;
        let t203 = t202 * t37;
        let t208 = t94 * t8;
        let t209 = t208 * t37;
        let tv3rhosigma20 = 0.50008333333333333333e-2 * t181 * t23 + 0.150025e-1 * t95 * t38 + 0.54009e-1 * t186 * t102 + 0.108018e0 * t101 * t190 - 0.27e1 * t193 * t108 - 0.1296216e0 * t197 * t198 - 0.486e1 * t201 * t203 - 0.15e0 * t79 * t112 - 0.9e0 * t88 * t209;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t212 = sigma[ip] * sigma[ip];
        let t214 = 1.0 / t1 / t212;
        let t215 = t18 * t214;
        let t218 = 1.0 / t212;
        let t219 = t99 * t218;
        let t222 = 1.0 / t63;
        let t223 = t222 * t94;
        let t226 = t222 * t7;
        let t227 = t130 * t94;
        let t231 = t58 * t218 * t107;
        let t235 = t28 * t214 * t8;
        let tv3sigma30 = 0.2250375e-1 * t215 * t23 + 0.810135e-1 * t219 * t102 + 0.1458243e0 * t223 * t198 - 0.8748e1 * t226 * t227 - 0.243e1 * t105 * t231 - 0.675e0 * t44 * t235;
        v3sigma3[ip] += tv3sigma30;
        let t248 = t37 * t71;
        let t253 = t139 * rho[ip];
        let t255 = 1.0 / t2 / t253;
        let t271 = 1.0 / t129 / t22;
        let t272 = t59 * t59;
        let t280 = t71 * t71;
        let tv4rho40 = 0.20744197530864197528e0 * t1 * t141 * t23 + 0.24893037037037037035e0 * t115 * t38 - 0.16002666666666666666e0 * t49 * t72 - 24.0 * t7 * t130 * t131 + 24.0 * t120 * t248 + 0.32005333333333333333e0 * t5 * t148 - t26 * t28 * (0.8088888888888888889e2 * t9 * t255 + 70.0 / 81.0 * t15 * t17 * t141) + 0.32005333333333333334e0 * t49 * t60 + 0.192032e1 * t5 * t132 - 0.192032e1 * t5 * t136 + 24.0 * t26 * t271 * t272 - 36.0 * t26 * t130 * t59 * t71 + 6.0 * t26 * t58 * t280 + 8.0 * t26 * t135 * t147 - 4.0 * t52 * t147;
        v4rho4[ip] += tv4rho40;
        let tv4rho3sigma0 = -0.31116296296296296297e-1 * t65 * t40 * t23 - 0.40006666666666666668e-1 * t150 * t38 + 0.30005000000000000001e-1 * t74 * t72 - 0.30005e-1 * t41 * t148 + 0.1296216e1 * t163 * t58 * t171 - 0.432072e0 * t170 * t8 * t71 + 0.93333333333333333333e0 * t65 * t7 * t46 + 0.24e1 * t159 * t58 * t90 - 0.18e1 * t167 * t178 + 0.18e1 * t88 * t89 * t147 + 0.54e1 * t79 * t130 * t175 + 0.216e2 * t44 * t271 * t89 * t131 - 0.162e2 * t174 * t89 * t248 - 0.60010000000000000001e-1 * t74 * t60 - 0.18003e0 * t41 * t132 + 0.18003e0 * t41 * t136 + 0.1296216e1 * t84 * t130 * t8 * t59 + 0.82413733333333333333e0 / t82 / t139 * t28 * t8;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t381 = t130 * t107 * t37;
        let tv4rho2sigma20 = -0.66677777777777777777e-2 * t31 * t94 * t23 - 0.324054e0 * t101 * t130 * t8 * t59 + 0.72e1 * t163 * t7 * t108 + 0.162e2 * t193 * t130 * t203 - 0.486e1 * t201 * t202 * t71 + 0.2e0 * t159 * t112 + 0.6e0 * t167 * t209 - 0.9e0 * t88 * t208 * t71 + 0.1944e2 * t105 * t271 * t202 * t59 + 0.27e1 * t174 * t208 * t59 - 0.30005e-1 * t95 * t60 - 0.132022e0 * t163 * t100 * t102 - 0.216036e0 * t186 * t190 + 0.108018e0 * t101 * t189 * t71 + 0.7777296e0 * t197 * t381 - 0.10001666666666666667e-1 * t181 * t38 + 0.150025e-1 * t95 * t72 + 0.7345224e0 / t253 * t40 * t198;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let tv4rhosigma30 = -0.750125e-2 * t4 * t214 * t23 - 0.2250375e-1 * t215 * t38 - 0.810135e-1 * t84 * t218 * t102 - 0.162027e0 * t219 * t190 - 0.2430405e0 * t196 * t94 * t198 - 0.4374729e0 * t223 * t381 + 0.26244e2 * t196 * t7 * t227 + 0.69995664e0 * t255 * t100 * t130 + 0.34992e2 * t226 * t271 * t94 * t37 + 0.405e1 * t193 * t231 + 0.729e1 * t201 * t218 * t107 * t37 + 0.225e0 * t79 * t235 + 0.135e1 * t88 * t214 * t8 * t37;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t429 = t212 * sigma[ip];
        let t431 = 1.0 / t1 / t429;
        let t435 = 1.0 / t429;
        let tv4sigma40 = -0.56259375e-1 * t18 * t431 * t23 - 0.20253375e0 * t99 * t435 * t102 - 0.4374729e0 * t222 * t214 * t198 - 0.104993496e1 * t141 * t218 * t130 + 0.314928e2 * t141 * t7 * t271 * t218 * t8 + 0.26244e2 * t226 * t130 * t214 + 0.6075e1 * t105 * t58 * t435 * t107 + 0.16875e1 * t44 * t28 * t431 * t8;
        v4sigma4[ip] += tv4sigma40;
    }
}
