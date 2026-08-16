//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 969/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk969(t174: f64, t361: f64, t1181: f64, t3361: f64, t3754: f64, t530: f64, t3730: f64, t14056: f64, t4912: f64, t3621: f64, t4640: f64, t1137: f64, t4787: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15695 = t361 * t174;
    let t15710 = t3361 * t1181 * t530 * t3754;
    let t15714 = t3361 * t1181 * t530 * t3730;
    let t15733 = t14056 * t4912;
    let t15746 = t3621 * t4640;
    let t15748 = t1137 * t4787;
    (t15695, t15710, t15714, t15733, t15746, t15748)
}
