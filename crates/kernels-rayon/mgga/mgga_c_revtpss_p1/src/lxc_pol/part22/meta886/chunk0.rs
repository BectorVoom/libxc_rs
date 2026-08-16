//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3072/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3072(t15199: f64, t698: f64, t2852: f64, t373: f64, t2439: f64, t4628: f64, t1606: f64, t9303: f64, t11387: f64, t4631: f64, t15513: f64, t914: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52065 = t698 * t15199;
    let t52110 = t373 * t2852;
    let t52126 = t2439 * t4628;
    let t52128 = t9303 * t1606;
    let t52163 = t4631 * t11387;
    let t52214 = t15513 * t914;
    (t52065, t52110, t52126, t52128, t52163, t52214)
}
