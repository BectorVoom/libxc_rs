//! GGA_C_WL lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_wl.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_wl_lxc_unpol(
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
        let t1 = rmath::sqrt(sigma[ip]);
        let t2 = pow_1_3(rho[ip]);
        let t4 = 1.0 / t2 / rho[ip];
        let t5 = t1 * t4;
        let t7 = -0.7486 + 0.06001 * t5;
        let t8 = M_CBRT2;
        let t9 = t1 * t8;
        let t12 = M_CBRT3;
        let t14 = pow_1_3(1.0 / M_PI);
        let t15 = t12 * t14;
        let t16 = M_CBRT4;
        let t17 = t16 * t16;
        let t18 = 1.0 / t2;
        let t22 = 3.60073 + 1.8 * t9 * t4 + t15 * t17 * t18 / 4.0;
        let t23 = 1.0 / t22;
        let tzk0 = t7 * t23;
        zk[ip] += tzk0;
        let t26 = rho[ip] * t7;
        let t27 = t22 * t22;
        let t28 = 1.0 / t27;
        let t29 = rho[ip] * rho[ip];
        let t31 = 1.0 / t2 / t29;
        let t37 = -2.4 * t9 * t31 - t15 * t17 * t4 / 12.0;
        let t38 = t28 * t37;
        let tvrho0 = tzk0 - 0.08001333333333334 * t5 * t23 - t26 * t38;
        vrho[ip] += tvrho0;
        let t40 = 1.0 / t1;
        let t41 = t18 * t40;
        let t44 = t18 * t7;
        let t46 = t28 * t40 * t8;
        let tvsigma0 = 0.030005 * t41 * t23 - 0.9 * t44 * t46;
        vsigma[ip] += tvsigma0;
        let t49 = t1 * t31;
        let t52 = t7 * t28;
        let t58 = 1.0 / t27 / t22;
        let t59 = t37 * t37;
        let t60 = t58 * t59;
        let t63 = t29 * rho[ip];
        let t65 = 1.0 / t2 / t63;
        let t71 = 5.6 * t9 * t65 + t15 * t17 * t31 / 9.0;
        let t72 = t28 * t71;
        let tv2rho20 = 0.02667111111111111 * t49 * t23 - 2.0 * t52 * t37 + 0.16002666666666668 * t5 * t38 + 2.0 * t26 * t60 - t26 * t72;
        v2rho2[ip] += tv2rho20;
        let t74 = t4 * t40;
        let t79 = t4 * t7;
        let t82 = t2 * t2;
        let t84 = 1.0 / t82 / t29;
        let t88 = t44 * t58;
        let t89 = t40 * t8;
        let t90 = t89 * t37;
        let tv2rhosigma0 = -0.010001666666666667 * t74 * t23 - 0.030005 * t41 * t38 + 0.3 * t79 * t46 + 0.072012 * t84 * t28 * t8 + 1.8 * t88 * t90;
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
        let tv2sigma20 = -0.0150025 * t95 * t23 - 0.054009 * t101 * t102 + 1.62 * t105 * t108 + 0.45 * t44 * t112;
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
        let t147 = -18.666666666666668 * t9 * t141 - 7.0 / 27.0 * t15 * t17 * t65;
        let t148 = t28 * t147;
        let tv3rho30 = -0.06223259259259259 * t115 * t23 - 0.08001333333333334 * t49 * t38 + 6.0 * t120 * t59 - 3.0 * t52 * t71 - 0.48008 * t5 * t60 + 0.24004 * t5 * t72 - 6.0 * t26 * t132 + 6.0 * t26 * t136 - t26 * t148;
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
        let tv3rho2sigma0 = 0.013335555555555555 * t150 * t23 + 0.020003333333333335 * t74 * t38 + 0.06001 * t41 * t60 - 0.030005 * t41 * t72 - 0.4 * t159 * t46 - 0.216036 * t163 * t28 * t8 - 1.2 * t167 * t90 - 0.288048 * t170 * t171 - 5.4 * t174 * t175 + 1.8 * t88 * t178;
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
        let tv3rhosigma20 = 0.005000833333333334 * t181 * t23 + 0.0150025 * t95 * t38 + 0.054009 * t186 * t102 + 0.108018 * t101 * t190 - 2.7 * t193 * t108 - 0.1296216 * t197 * t198 - 4.86 * t201 * t203 - 0.15 * t79 * t112 - 0.9 * t88 * t209;
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
        let tv3sigma30 = 0.02250375 * t215 * t23 + 0.0810135 * t219 * t102 + 0.1458243 * t223 * t198 - 8.748 * t226 * t227 - 2.43 * t105 * t231 - 0.675 * t44 * t235;
        v3sigma3[ip] += tv3sigma30;
        let t248 = t37 * t71;
        let t253 = t139 * rho[ip];
        let t255 = 1.0 / t2 / t253;
        let t271 = 1.0 / t129 / t22;
        let t272 = t59 * t59;
        let t280 = t71 * t71;
        let tv4rho40 = 0.20744197530864197 * t1 * t141 * t23 + 0.24893037037037036 * t115 * t38 - 0.16002666666666668 * t49 * t72 - 24.0 * t7 * t130 * t131 + 24.0 * t120 * t248 + 0.32005333333333336 * t5 * t148 - t26 * t28 * (80.88888888888889 * t9 * t255 + 70.0 / 81.0 * t15 * t17 * t141) + 0.32005333333333336 * t49 * t60 + 1.92032 * t5 * t132 - 1.92032 * t5 * t136 + 24.0 * t26 * t271 * t272 - 36.0 * t26 * t130 * t59 * t71 + 6.0 * t26 * t58 * t280 + 8.0 * t26 * t135 * t147 - 4.0 * t52 * t147;
        v4rho4[ip] += tv4rho40;
        let tv4rho3sigma0 = -0.031116296296296295 * t65 * t40 * t23 - 0.04000666666666667 * t150 * t38 + 0.030005 * t74 * t72 - 0.030005 * t41 * t148 + 1.296216 * t163 * t58 * t171 - 0.432072 * t170 * t8 * t71 + 0.9333333333333333 * t65 * t7 * t46 + 2.4 * t159 * t58 * t90 - 1.8 * t167 * t178 + 1.8 * t88 * t89 * t147 + 5.4 * t79 * t130 * t175 + 21.6 * t44 * t271 * t89 * t131 - 16.2 * t174 * t89 * t248 - 0.06001 * t74 * t60 - 0.18003 * t41 * t132 + 0.18003 * t41 * t136 + 1.296216 * t84 * t130 * t8 * t59 + 0.8241373333333334 / t82 / t139 * t28 * t8;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t381 = t130 * t107 * t37;
        let tv4rho2sigma20 = -0.006667777777777778 * t31 * t94 * t23 - 0.324054 * t101 * t130 * t8 * t59 + 7.2 * t163 * t7 * t108 + 16.2 * t193 * t130 * t203 - 4.86 * t201 * t202 * t71 + 0.2 * t159 * t112 + 0.6 * t167 * t209 - 0.9 * t88 * t208 * t71 + 19.44 * t105 * t271 * t202 * t59 + 2.7 * t174 * t208 * t59 - 0.030005 * t95 * t60 - 0.132022 * t163 * t100 * t102 - 0.216036 * t186 * t190 + 0.108018 * t101 * t189 * t71 + 0.7777296 * t197 * t381 - 0.010001666666666667 * t181 * t38 + 0.0150025 * t95 * t72 + 0.7345224 / t253 * t40 * t198;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let tv4rhosigma30 = -0.00750125 * t4 * t214 * t23 - 0.02250375 * t215 * t38 - 0.0810135 * t84 * t218 * t102 - 0.162027 * t219 * t190 - 0.2430405 * t196 * t94 * t198 - 0.4374729 * t223 * t381 + 26.244 * t196 * t7 * t227 + 0.69995664 * t255 * t100 * t130 + 34.992 * t226 * t271 * t94 * t37 + 4.05 * t193 * t231 + 7.29 * t201 * t218 * t107 * t37 + 0.225 * t79 * t235 + 1.35 * t88 * t214 * t8 * t37;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t429 = t212 * sigma[ip];
        let t431 = 1.0 / t1 / t429;
        let t435 = 1.0 / t429;
        let tv4sigma40 = -0.056259375 * t18 * t431 * t23 - 0.20253375 * t99 * t435 * t102 - 0.4374729 * t222 * t214 * t198 - 1.04993496 * t141 * t218 * t130 + 31.4928 * t141 * t7 * t271 * t218 * t8 + 26.244 * t226 * t130 * t214 + 6.075 * t105 * t58 * t435 * t107 + 1.6875 * t44 * t28 * t431 * t8;
        v4sigma4[ip] += tv4sigma40;
    }
}
