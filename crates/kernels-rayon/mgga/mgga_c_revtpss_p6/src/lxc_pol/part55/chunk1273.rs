//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1273/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1273(t2042: f64, t28956: f64, t2113: f64, t28271: f64, t28277: f64, t28974: f64, t572: f64, t7741: f64, t26733: f64, t1459: f64, t34366: f64, t28265: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t129080 = 3.0_f64 * t28956 * t2042;
    let t129082 = 6.0_f64 * t2113 * t28271;
    let t129084 = 6.0_f64 * t2113 * t28277;
    let t129089 = 6.0_f64 * t572 * t28974 * t7741;
    let t129092 = 6.0_f64 * t572 * t26733 * t7741;
    let t129095 = 6.0_f64 * t1459 * t34366;
    let t129097 = 6.0_f64 * t2113 * t28265;
    (t129080, t129082, t129084, t129089, t129092, t129095, t129097)
}
