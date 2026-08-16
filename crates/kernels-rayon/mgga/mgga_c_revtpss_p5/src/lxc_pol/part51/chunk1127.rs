//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1127/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1127(t125939: f64, t28196: f64, t28197: f64, t125531: f64, t125532: f64, t125536: f64, t125537: f64, t125539: f64, t125541: f64, t125543: f64, t125545: f64, t125547: f64, t125550: f64, t125552: f64, t125554: f64, t125556: f64, t125558: f64, t125562: f64, t125566: f64, t125938: f64) -> f64 {
    let t125942 = 4.0_f64 * t28196 * t28197 * t125939;
    let t125943 = -t125531 - 6.0_f64 * t125532 + t125536 - 4.0_f64 * t125537 - 4.0_f64 * t125539 - 4.0_f64 * t125541 - 4.0_f64 * t125543 - 4.0_f64 * t125545 - 4.0_f64 * t125547 - 4.0_f64 * t125550 - 4.0_f64 * t125552 - 4.0_f64 * t125554 - 4.0_f64 * t125556 + t125558 - t125562 + t125566 + t125938 + t125942;
    t125943
}
