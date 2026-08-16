//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 508/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk508(t366: f64, t4857: f64, t1065: f64, t905: f64, t1032: f64, t1647: f64, t1040: f64, t3147: f64, t72: f64, t3088: f64, t3299: f64, t1668: f64, t3153: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4858 = t4857 * t366;
    let t4872 = t1065 * t905;
    let t4878 = t1647 * t1032;
    let t4879 = t4878 * t1040;
    let t4890 = t3147 * t72;
    let t4891 = t3088 * t4890;
    let t4892 = t3299 * t4891;
    let t4893 = t1668 * t3153;
    (t4858, t4872, t4879, t4890, t4891, t4892, t4893)
}
