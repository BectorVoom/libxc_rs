//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2128/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2128(t2322: f64, t2371: f64, t25078: f64, t25805: f64, t28050: f64, t4248: f64, t4254: f64, t4257: f64, t4292: f64, t651: f64, t7221: f64, t7883: f64, t97639: f64, t97641: f64, t97643: f64, t97645: f64, t97647: f64, t97649: f64, t97653: f64, t97657: f64, t97659: f64, t97661: f64, t97663: f64, t97666: f64, t98421: f64) -> f64 {
    let t98422 = -2.0_f64 * t2371 * t651 * t7883 - 4.0_f64 * t4292 * t651 * t7221 - 4.0_f64 * t2322 * t28050 - 2.0_f64 * t25078 * t4248 - 4.0_f64 * t25805 * t4257 - 4.0_f64 * t28050 * t4254 - t97639 - t97641 - t97643 - t97645 - t97647 - t97649 + t97653 + t97657 + t97659 + t97661 - t97663 - t97666 + t98421;
    t98422
}
