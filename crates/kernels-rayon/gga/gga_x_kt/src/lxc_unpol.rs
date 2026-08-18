//! GGA_X_KT lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_kt.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_kt_lxc_unpol(
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
    param_gamma: f64,
    param_delta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t5 = 1.0 / t4;
        let t6 = t3 * t5;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = t3 * t3;
        let t21 = param_gamma * t20;
        let t23 = pow_1_3(1.0 / M_PI);
        let t24 = 1.0 / t23;
        let t25 = M_CBRT4;
        let t26 = t24 * t25;
        let t27 = t21 * t26;
        let t28 = M_CBRT2;
        let t29 = t11 * rho[ip];
        let t30 = pow_1_3(t29);
        let t31 = t30 * t29;
        let t32 = t28 * t31;
        let t33 = rho[ip] * rho[ip];
        let t34 = t18 * t18;
        let t36 = 1.0 / t34 / t33;
        let t37 = sigma[ip] * t36;
        let t38 = t28 * t28;
        let t41 = t38 * t31 / 4.0 + param_delta;
        let t42 = 1.0 / t41;
        let t47 = 1.0 - t27 * t32 * t37 * t42 / 9.0;
        let t51 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t47);
        let tzk0 = 2.0 * t51;
        zk[ip] += tzk0;
        let t52 = 1.0 / t34;
        let t53 = t17 * t52;
        let t58 = t21 * t26 * t28;
        let t59 = t30 * sigma[ip];
        let t60 = t36 * t42;
        let t65 = t33 * rho[ip];
        let t67 = 1.0 / t34 / t65;
        let t73 = t30 * t30;
        let t74 = t73 * t29;
        let t75 = t74 * sigma[ip];
        let t76 = t41 * t41;
        let t77 = 1.0 / t76;
        let t78 = t36 * t77;
        let t83 = -4.0 / 27.0 * t58 * t59 * t60 * t11 + 8.0 / 27.0 * t27 * t32 * sigma[ip] * t67 * t42 + 2.0 / 27.0 * t27 * t75 * t78 * t11;
        let t88 = piecewise3(t2, 0.0, -t6 * t53 * t47 / 8.0 - 3.0 / 8.0 * t6 * t19 * t83);
        let tvrho0 = 2.0 * rho[ip] * t88 + 2.0 * t51;
        vrho[ip] += tvrho0;
        let t91 = t5 * t17;
        let t94 = 1.0 / t18 / t33 * param_gamma;
        let t95 = t91 * t94;
        let t97 = t26 * t32 * t42;
        let t100 = piecewise3(t2, 0.0, t95 * t97 / 8.0);
        let tvsigma0 = 2.0 * rho[ip] * t100;
        vsigma[ip] += tvsigma0;
        let t104 = 1.0 / t34 / rho[ip];
        let t105 = t17 * t104;
        let t112 = 1.0 / t73;
        let t113 = t112 * sigma[ip];
        let t114 = t11 * t11;
        let t119 = t67 * t42;
        let t124 = t73 * sigma[ip];
        let t129 = t33 * t33;
        let t131 = 1.0 / t34 / t129;
        let t137 = t67 * t77;
        let t142 = t114 * t114;
        let t145 = 1.0 / t76 / t41;
        let t147 = sigma[ip] * t145 * t38;
        let t151 = -4.0 / 81.0 * t58 * t113 * t60 * t114 + 64.0 / 81.0 * t58 * t59 * t119 * t11 + 2.0 / 9.0 * t27 * t124 * t78 * t114 - 88.0 / 81.0 * t27 * t32 * sigma[ip] * t131 * t42 - 32.0 / 81.0 * t27 * t75 * t137 * t11 - 4.0 / 81.0 * t27 * t142 * t52 * t147;
        let t156 = piecewise3(t2, 0.0, t6 * t105 * t47 / 12.0 - t6 * t53 * t83 / 4.0 - 3.0 / 8.0 * t6 * t19 * t151);
        let tv2rho20 = 2.0 * rho[ip] * t156 + 4.0 * t88;
        v2rho2[ip] += tv2rho20;
        let t161 = 1.0 / t18 / t65 * param_gamma;
        let t162 = t91 * t161;
        let t166 = t91 * t94 * t24;
        let t167 = t25 * t28;
        let t170 = t167 * t30 * t42 * t11;
        let t175 = t26 * t74 * t77 * t11;
        let t179 = piecewise3(t2, 0.0, -7.0 / 24.0 * t162 * t97 + t166 * t170 / 6.0 - t95 * t175 / 12.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t179 + 2.0 * t100;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2sigma20 = 0.0;
        v2sigma2[ip] += tv2sigma20;
        let t183 = t17 * t36;
        let t193 = 1.0 / t74;
        let t194 = t193 * sigma[ip];
        let t195 = t114 * t11;
        let t204 = 1.0 / t30;
        let t205 = t204 * sigma[ip];
        let t210 = t131 * t42;
        let t223 = t129 * rho[ip];
        let t225 = 1.0 / t34 / t223;
        let t231 = t131 * t77;
        let t236 = t142 * t11;
        let t238 = t21 * t26 * t236;
        let t239 = t52 * sigma[ip];
        let t240 = t76 * t76;
        let t242 = 1.0 / t240 * t28;
        let t243 = t242 * t30;
        let t247 = 8.0 / 243.0 * t58 * t194 * t60 * t195 + 32.0 / 81.0 * t58 * t113 * t119 * t114 + 44.0 / 243.0 * t27 * t205 * t78 * t195 - 352.0 / 81.0 * t58 * t59 * t210 * t11 - 16.0 / 9.0 * t27 * t124 * t137 * t114 + 4.0 / 27.0 * t27 * t142 * t104 * t147 + 1232.0 / 243.0 * t27 * t32 * sigma[ip] * t225 * t42 + 176.0 / 81.0 * t27 * t75 * t231 * t11 + 8.0 / 81.0 * t238 * t239 * t243;
        let t252 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t183 * t47 + t6 * t105 * t83 / 4.0 - 3.0 / 8.0 * t6 * t53 * t151 - 3.0 / 8.0 * t6 * t19 * t247);
        let tv3rho30 = 2.0 * rho[ip] * t252 + 6.0 * t156;
        v3rho3[ip] += tv3rho30;
        let t258 = 1.0 / t18 / t129 * param_gamma;
        let t259 = t91 * t258;
        let t263 = t91 * t161 * t24;
        let t270 = t167 * t112 * t42 * t114;
        let t275 = t26 * t73 * t77 * t114;
        let t279 = 1.0 / t18 * param_gamma;
        let t282 = t142 * t145 * t38;
        let t283 = t26 * t282;
        let t287 = piecewise3(t2, 0.0, 35.0 / 36.0 * t259 * t97 - 7.0 / 9.0 * t263 * t170 + 7.0 / 18.0 * t162 * t175 + t166 * t270 / 18.0 - t95 * t275 / 4.0 + t91 * t279 * t283 / 18.0);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t287 + 4.0 * t179;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let tv3rhosigma20 = 0.0;
        v3rhosigma2[ip] += tv3rhosigma20;
        let tv3sigma30 = 0.0;
        v3sigma3[ip] += tv3sigma30;
        let t316 = t142 * t114;
        let t374 = -20944.0 / 729.0 * t27 * t32 * sigma[ip] / t34 / t129 / t33 * t42 - 88.0 / 243.0 * t238 * t104 * sigma[ip] * t243 + 8.0 / 243.0 * t21 * t26 * t316 * t239 * t242 * t112 + 19712.0 / 729.0 * t58 * t59 * t225 * t42 * t11 - 40.0 / 729.0 * t58 / t73 / t114 / t33 * sigma[ip] * t60 * t142 - 256.0 / 729.0 * t58 * t194 * t119 * t195 - 704.0 / 243.0 * t58 * t113 * t210 * t114 - 20.0 / 243.0 * t27 / t31 * sigma[ip] * t78 * t142 - 1408.0 / 729.0 * t27 * t205 * t137 * t195 - 460.0 / 729.0 * t27 * t37 * t282 + 352.0 / 27.0 * t27 * t124 * t231 * t114 - 9856.0 / 729.0 * t27 * t75 * t225 * t77 * t11 - 64.0 / 243.0 * t27 * t316 * t52 * sigma[ip] / t240 / t41 * t73;
        let t379 = piecewise3(t2, 0.0, 10.0 / 27.0 * t6 * t17 * t67 * t47 - 5.0 / 9.0 * t6 * t183 * t83 + t6 * t105 * t151 / 2.0 - t6 * t53 * t247 / 2.0 - 3.0 / 8.0 * t6 * t19 * t374);
        let tv4rho40 = 2.0 * rho[ip] * t379 + 8.0 * t252;
        v4rho4[ip] += tv4rho40;
        let t422 = piecewise3(t2, 0.0, -455.0 / 108.0 * t91 / t18 / t223 * param_gamma * t97 + 35.0 / 9.0 * t91 * t258 * t24 * t170 - 35.0 / 18.0 * t259 * t175 - 7.0 / 18.0 * t263 * t270 + 7.0 / 4.0 * t162 * t275 - t91 / t18 / rho[ip] * param_gamma * t283 / 9.0 - t166 * t167 * t193 * t42 * t195 / 27.0 - 11.0 / 54.0 * t95 * t26 * t204 * t77 * t195 - t91 * t279 * t24 * t25 * t236 * t243 / 9.0);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t422 + 6.0 * t287;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let tv4rho2sigma20 = 0.0;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let tv4rhosigma30 = 0.0;
        v4rhosigma3[ip] += tv4rhosigma30;
        let tv4sigma40 = 0.0;
        v4sigma4[ip] += tv4sigma40;
    }
}
