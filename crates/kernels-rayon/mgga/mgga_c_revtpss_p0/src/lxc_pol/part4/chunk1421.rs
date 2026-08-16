//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1421/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1421(t18004: f64, t18040: f64, t18080: f64, t18121: f64, t1300: f64, t16641: f64, t16645: f64, t16647: f64, t16649: f64, t16651: f64, t16654: f64, t16657: f64, t16660: f64, t16664: f64, t16667: f64, t16671: f64, t16675: f64, t16679: f64, t16681: f64, t16684: f64, t16687: f64, t16690: f64, t16783: f64, t198: f64, t336: f64) -> f64 {
    let t18123 = t18004 + t18040 + t18080 + t18121;
    let t18127 = t1300 * t18123 * t198 * t336 - t16641 + t16645 + t16647 - t16649 + t16651 - t16654 - t16657 - t16660 + t16664 + t16667 + t16671 - t16675 + t16679 - t16681 + t16684 - t16687 + t16690 - t16783;
    t18127
}
