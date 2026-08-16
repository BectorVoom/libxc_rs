//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1172/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1172(t110322: f64, t25387: f64, t30380: f64, t686: f64, t72: f64, t7058: f64, t28314: f64, t99466: f64, t7064: f64, t103067: f64, t4481: f64, t27216: f64, t28360: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t110323 = t25387 * t110322;
    let t110339 = t30380 * t72 * t686;
    let t110340 = t7058 * t110339;
    let t110344 = t99466 * t28314;
    let t110346 = t7064 * t110339;
    let t110355 = t103067 * t4481;
    let t110453 = t27216 * t28360;
    (t110323, t110340, t110344, t110346, t110355, t110453)
}
