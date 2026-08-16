//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1013/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1013(t2243: f64, t8219: f64, t7930: f64, t6090: f64, t6093: f64, t6348: f64, t7947: f64, t7955: f64, t6088: f64, t352: f64, t2259: f64, t3088: f64, t365: f64, t6272: f64, t8191: f64, t8194: f64, t8197: f64, t8201: f64, t8204: f64, t8208: f64, t8211: f64, t8216: f64, t8218: f64) -> (f64, f64, f64, f64, f64) {
    let t8221 = 0.16081979498692535067e2_f64 * t8219 * t2243;
    let t8225 = 0.34246666666666666666e-1_f64 * t7930;
    let t8227 = -t6348 + 0.45662222222222222222e-1_f64 * t6090 - 0.17123333333333333333e-1_f64 * t6093 + 0.22831111111111111111e-1_f64 * t7955 - t8225 + 0.5137e-1_f64 * t7947;
    let t8233 = 0.35616666666666666666e-1_f64 * t7930;
    let t8235 = -t6088 + 0.47488888888888888888e-1_f64 * t6090 - 0.17808333333333333333e-1_f64 * t6093 + 0.23744444444444444444e-1_f64 * t7955 - t8233 + 0.53425e-1_f64 * t7947;
    let t8237 = 0.621814e-1_f64 * t8235 * t352;
    let t8238 = t8191 + t8194 + t8197 - t8201 - t8204 - t8208 - 4.0_f64 * t6272 * t3088 - 2.0_f64 * t8211 * t2259 - t8216 - t8218 - t8221 - 0.310907e-1_f64 * t8227 * t365 + t8237;
    (t8221, t8227, t8235, t8237, t8238)
}
