//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2276/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2276<F: Float>(t1280: F, t16775: F, t16641: F, t16645: F, t16647: F, t16649: F, t16651: F, t16654: F, t16657: F, t16660: F, t16664: F, t16667: F, t16671: F, t16675: F, t16679: F, t16681: F, t16684: F, t16687: F, t16690: F) -> (F, F) {
    let t16776 = t1280 * t16775;
    let t16781 = -t16641 + t16645 + t16647 - t16649 + t16651 - t16654 - t16657 - t16660 + t16664 + t16667 + t16671 - t16675 + t16679 - t16681 + t16684 - t16687 + t16690;
    (t16776, t16781)
}
