//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 964/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk964(t7306: f64, t7987: f64, t2122: f64, t2132: f64, t7885: f64, t864: f64, t1219: f64, t615: f64, t7911: f64, t862: f64, t865: f64, t15407: f64, t7942: f64, t9033: f64) -> (f64, f64, f64, f64, f64) {
    let t31951 = t7987 * t7306;
    let t31955 = t7885 * t2132 * t2122 * t864;
    let t31965 = t615 * t7911 * t1219;
    let t31969 = t862 * t2122 * t865;
    let t31972 = t7942 * t9033 * t15407;
    (t31951, t31955, t31965, t31969, t31972)
}
