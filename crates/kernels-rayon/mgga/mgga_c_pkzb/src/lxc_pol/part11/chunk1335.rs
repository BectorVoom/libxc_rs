//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1335/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1335(t10283: f64, t10297: f64, t10300: f64, t10306: f64, t11484: f64, t1246: f64, t1256: f64, t1259: f64, t1306: f64, t135: f64, t158: f64, t273: f64, t28595: f64, t31456: f64, t31458: f64, t31461: f64, t31464: f64, t31957: f64, t31960: f64, t31962: f64, t31965: f64, t31967: f64, t32225: f64, t32400: f64, t3247: f64, t3255: f64, t3279: f64, t3904: f64, t3910: f64, t3929: f64, t415: f64, t952: f64, t957: f64) -> f64 {
    let t32408 = t31456 - t31458 - t31461 + t31464 + t135 * t273 * (0.65854491829355115987e0_f64 * t32225 * t158 * t415 - 0.65854491829355115987e0_f64 * t11484 * t952 - 0.19756347548806534796e1_f64 * t10283 * t1256 + 0.39512695097613069592e1_f64 * t3904 * t3255 - 0.19756347548806534796e1_f64 * t3904 * t3279 + 0.39512695097613069591e1_f64 * t3247 * t3910 - 0.11853808529283920877e2_f64 * t1246 * t10297 + 0.79025390195226139182e1_f64 * t1246 * t10300 - 0.19756347548806534796e1_f64 * t3247 * t3929 + 0.39512695097613069592e1_f64 * t1246 * t10306 + t32400) * t957 - t31957 + t31960 + t31962 - t31965 - 3.0_f64 * t1306 * t28595 * t1259 - t31967;
    t32408
}
