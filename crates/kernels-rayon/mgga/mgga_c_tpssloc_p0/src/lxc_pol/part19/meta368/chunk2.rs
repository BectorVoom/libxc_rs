//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1358/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1358(t1032: f64, t10375: f64, t370: f64, t374: f64, t376: f64, t9697: f64, t10908: f64, t3109: f64, t1036: f64, t10446: f64, t1004: f64, t10249: f64, t10413: f64, t10445: f64, t14220: f64, t2979: f64, t3070: f64, t3071: f64, t35: f64, t354: f64, t364: f64, t378: f64, t41649: f64, t43226: f64, t43228: f64, t43233: f64, t43235: f64, t43241: f64, t43246: f64, t6720: f64, t973: f64) -> f64 {
    let t43248 = t1032 * t10375;
    let t43253 = 7.0_f64 / 31104.0_f64 * t370 * t374 * t9697 * t376;
    let t43254 = t3109 * t10908;
    let t43262 = t10446 * t1036;
    let t43267 = t43226 / 576.0_f64 + t43228 / 216.0_f64 - t973 * t2979 * t41649 / 6.0_f64 - t43233 / 384.0_f64 - t10413 * t3071 * t43235 * t14220 / 384.0_f64 - t3070 * t3071 * t10249 * t43241 / 192.0_f64 - t43246 / 72.0_f64 - t43248 / 486.0_f64 - t43253 - t43254 / 72.0_f64 + 5225.0_f64 / 7776.0_f64 * t354 * t364 / t35 / t6720 * t378 - 209.0_f64 / 972.0_f64 * t43262 - 209.0_f64 / 648.0_f64 * t1004 * t10445 * t378;
    t43267
}
