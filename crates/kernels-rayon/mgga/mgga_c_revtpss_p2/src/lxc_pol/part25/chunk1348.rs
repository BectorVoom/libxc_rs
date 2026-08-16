//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1348/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1348(t2014: f64, t26089: f64, t7315: f64, t28196: f64, t28197: f64, t49654: f64, t1450: f64, t9628: f64, t7237: f64, t25082: f64, t49560: f64, t3813: f64, t651: f64, t7002: f64) -> (f64, f64, f64, f64, f64) {
    let t94998 = 3.0_f64 * t2014 * t26089 * t7315;
    let t95001 = 6.0_f64 * t28196 * t28197 * t49654;
    let t95002 = t1450 * t9628;
    let t95005 = 3.0_f64 * t2014 * t7237 * t95002;
    let t95008 = 18.0_f64 * t25082 * t28197 * t49560;
    let t95011 = 6.0_f64 * t651 * t3813 * t7002;
    (t94998, t95001, t95005, t95008, t95011)
}
