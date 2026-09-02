//! GGA_X_G96 lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_g96.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_g96_lxc_unpol(
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
        let t20 = t3 * t3;
        let t22 = pow_1_3(1.0 / M_PI);
        let t23 = 1.0 / t22;
        let t25 = M_CBRT4;
        let t26 = rmath::sqrt(sigma[ip]);
        let t27 = M_CBRT2;
        let t28 = t26 * t27;
        let t31 = t28 / t18 / rho[ip];
        let t32 = rmath::sqrt(t31);
        let t33 = t32 * t31;
        let t37 = 1.0 + 2.0 / 1233.0 * t20 * t23 * t25 * t33;
        let t41 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t17 * t18 * t37);
        let tzk0 = 2.0 * t41;
        zk[ip] += tzk0;
        let t42 = t18 * t18;
        let t48 = t5 * t17;
        let t49 = rho[ip] * rho[ip];
        let t52 = t48 / t49 * t23;
        let t53 = t25 * t32;
        let t54 = t53 * t28;
        let t58 = piecewise3(t2, 0.0, -t6 * t17 / t42 * t37 / 8.0 + t52 * t54 / 274.0);
        let tvrho0 = 2.0 * rho[ip] * t58 + 2.0 * t41;
        vrho[ip] += tvrho0;
        let t63 = t48 / rho[ip] * t23;
        let t64 = 1.0 / t26;
        let t66 = t53 * t64 * t27;
        let t69 = piecewise3(t2, 0.0, -3.0 / 2192.0 * t63 * t66);
        let tvsigma0 = 2.0 * rho[ip] * t69;
        vsigma[ip] += tvsigma0;
        let t78 = t49 * rho[ip];
        let t81 = t48 / t78 * t23;
        let t84 = t49 * t49;
        let t86 = 1.0 / t18 / t84;
        let t88 = t48 * t86 * t23;
        let t89 = 1.0 / t32;
        let t90 = t25 * t89;
        let t91 = t27 * t27;
        let t92 = sigma[ip] * t91;
        let t93 = t90 * t92;
        let t97 = piecewise3(t2, 0.0, t6 * t17 / t42 / rho[ip] * t37 / 12.0 - 5.0 / 822.0 * t81 * t54 - t88 * t93 / 411.0);
        let tv2rho20 = 2.0 * rho[ip] * t97 + 4.0 * t58;
        v2rho2[ip] += tv2rho20;
        let t103 = 1.0 / t18 / t78;
        let t105 = t23 * t25;
        let t107 = t105 * t89 * t91;
        let t111 = piecewise3(t2, 0.0, 3.0 / 2192.0 * t52 * t66 + t48 * t103 * t107 / 1096.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t111 + 2.0 * t69;
        v2rhosigma[ip] += tv2rhosigma0;
        let t117 = t48 / t18 / t49 * t23;
        let t118 = 1.0 / sigma[ip];
        let t120 = t90 * t118 * t91;
        let t123 = t26 * sigma[ip];
        let t124 = 1.0 / t123;
        let t126 = t53 * t124 * t27;
        let t130 = piecewise3(t2, 0.0, -3.0 / 8768.0 * t117 * t120 + 3.0 / 4384.0 * t63 * t126);
        let tv2sigma20 = 2.0 * rho[ip] * t130;
        v2sigma2[ip] += tv2sigma20;
        let t134 = 1.0 / t42 / t49;
        let t141 = t48 / t84 * t23;
        let t144 = t84 * rho[ip];
        let t146 = 1.0 / t18 / t144;
        let t151 = t84 * t49;
        let t154 = t48 / t42 / t151;
        let t155 = 1.0 / t33;
        let t157 = t105 * t155 * t123;
        let t161 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t17 * t134 * t37 + 43.0 / 2466.0 * t141 * t54 + 2.0 / 137.0 * t48 * t146 * t23 * t93 - 4.0 / 1233.0 * t154 * t157);
        let tv3rho30 = 2.0 * rho[ip] * t161 + 6.0 * t97;
        v3rho3[ip] += tv3rho30;
        let t172 = t48 / t42 / t144;
        let t174 = t105 * t155 * t26;
        let t178 = piecewise3(t2, 0.0, -3.0 / 1096.0 * t81 * t66 - 13.0 / 3288.0 * t48 * t86 * t107 + t172 * t174 / 822.0);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t178 + 4.0 * t111;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t182 = t48 * t103 * t23;
        let t187 = t48 / t42 / t84;
        let t189 = t105 * t155 * t64;
        let t195 = piecewise3(t2, 0.0, 3.0 / 8768.0 * t182 * t120 - t187 * t189 / 2192.0 - 3.0 / 4384.0 * t52 * t126);
        let tv3rhosigma20 = 2.0 * rho[ip] * t195 + 2.0 * t130;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t199 = 1.0 / t42 / t78;
        let t200 = t48 * t199;
        let t202 = t105 * t155 * t124;
        let t205 = sigma[ip] * sigma[ip];
        let t206 = 1.0 / t205;
        let t208 = t90 * t206 * t91;
        let t212 = 1.0 / t26 / t205;
        let t214 = t53 * t212 * t27;
        let t218 = piecewise3(t2, 0.0, 3.0 / 17536.0 * t200 * t202 + 9.0 / 17536.0 * t117 * t208 - 9.0 / 8768.0 * t63 * t214);
        let tv3sigma30 = 2.0 * rho[ip] * t218;
        v3sigma3[ip] += tv3sigma30;
        let t227 = t48 / t144 * t23;
        let t236 = t84 * t78;
        let t242 = t84 * t84;
        let t249 = 1.0 / t32 / t92 / t134;
        let t250 = t25 * t249;
        let t256 = piecewise3(t2, 0.0, 10.0 / 27.0 * t6 * t17 * t199 * t37 - 253.0 / 3699.0 * t227 * t54 - 331.0 / 3699.0 * t48 / t18 / t151 * t23 * t93 + 152.0 / 3699.0 * t48 / t42 / t236 * t157 - 8.0 / 1233.0 * t48 / t242 / rho[ip] * t23 * t250 * t205 * t27);
        let tv4rho40 = 2.0 * rho[ip] * t256 + 8.0 * t161;
        v4rho4[ip] += tv4rho40;
        let t275 = piecewise3(t2, 0.0, 9.0 / 1096.0 * t141 * t66 + 187.0 / 9864.0 * t48 * t146 * t107 - 5.0 / 411.0 * t154 * t174 + t48 / t242 * t23 * t250 * sigma[ip] * t27 / 411.0);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t275 + 6.0 * t178;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t292 = piecewise3(t2, 0.0, -3.0 / 4384.0 * t88 * t120 + 17.0 / 6576.0 * t172 * t189 - t48 / t236 * t105 * t249 * t27 / 1096.0 + 3.0 / 2192.0 * t81 * t126);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t292 + 4.0 * t195;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t309 = piecewise3(t2, 0.0, t187 * t202 / 17536.0 + 3.0 / 8768.0 * t48 / t151 * t23 * t250 * t118 * t27 - 9.0 / 17536.0 * t182 * t208 + 9.0 / 8768.0 * t52 * t214);
        let tv4rhosigma30 = 2.0 * rho[ip] * t309 + 2.0 * t218;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t320 = t205 * sigma[ip];
        let t333 = piecewise3(t2, 0.0, -9.0 / 70144.0 * t227 * t250 * t206 * t27 - 9.0 / 17536.0 * t200 * t105 * t155 * t212 - 45.0 / 35072.0 * t117 * t90 / t320 * t91 + 45.0 / 17536.0 * t63 * t53 / t26 / t320 * t27);
        let tv4sigma40 = 2.0 * rho[ip] * t333;
        v4sigma4[ip] += tv4sigma40;
    }
}
