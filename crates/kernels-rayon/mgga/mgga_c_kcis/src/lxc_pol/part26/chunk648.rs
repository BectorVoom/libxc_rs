//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 648/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk648(t1546: f64, t7296: f64, t4293: f64, t6917: f64, t4292: f64, t2039: f64, t6016: f64, t584: f64, t7257: f64, t583: f64, t7276: f64, t7278: f64, t7280: f64, t7284: f64, t7288: f64, t7290: f64, t7292: f64, t7294: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7297 = t1546 * t7296;
    let t7299 = t4293 * t6917;
    let t7300 = t4292 * t7299;
    let t7302 = t6016 * t2039;
    let t7304 = t584 * t7257;
    let t7305 = t583 * t7304;
    let t7306 = t1546 * t7305;
    let t7308 = -t7276 / 576.0_f64 - t7278 / 3.0_f64 + t7280 / 12.0_f64 - t7284 / 16.0_f64 - t7288 / 192.0_f64 + t7290 / 24.0_f64 - t7292 / 96.0_f64 + t7294 / 128.0_f64 - t7297 / 24.0_f64 + t7300 / 96.0_f64 - t7302 / 8.0_f64 + t7306 / 256.0_f64;
    (t7297, t7299, t7300, t7302, t7305, t7306, t7308)
}
