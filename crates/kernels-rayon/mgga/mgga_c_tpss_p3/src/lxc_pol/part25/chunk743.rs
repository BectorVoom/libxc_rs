//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 743/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk743(t4907: f64, t885: f64, t2577: f64, t4891: f64, t2581: f64, t3746: f64, t4828: f64, t4832: f64, t4836: f64, t318: f64, t1448: f64) -> (f64, f64, f64, f64, f64) {
    let t4908 = t4907 * t885;
    let t4911 = t4891 * t2577;
    let t4918 = t2581 + 0.61805555555555555556e-2_f64 * t3746 - 0.61805555555555555555e-2_f64 * t4828 + 0.18541666666666666667e-1_f64 * t4832 - 0.92708333333333333333e-2_f64 * t4836;
    let t4919 = t4918 * t318;
    let t4923 = t1448 * t1448;
    (t4908, t4911, t4918, t4919, t4923)
}
