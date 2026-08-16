//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 763/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk763(t572: f64, t7953: f64, t1918: f64, t2040: f64, t573: f64, t7944: f64, t7949: f64, t7952: f64, t1469: f64, t1479: f64, t61: f64, t6971: f64, t7571: f64) -> (f64, f64, f64) {
    let t7955 = 3.0_f64 * t572 * t7953;
    let t7956 = 3.0_f64 * t1918 * t2040 + t573 * t7944 + t7949 + t7952 + t7955;
    let t8142 = -8.0_f64 / 3.0_f64 * t1479 * t61 - 5.0_f64 / 6.0_f64 * t7571 * t1469 + t6971;
    (t7955, t7956, t8142)
}
