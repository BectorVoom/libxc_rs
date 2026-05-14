//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1246/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1246<F: Float>(t1280: F, t16750: F, t3153: F, t5284: F, t5465: F, t1287: F, t1811: F, t3588: F, t13133: F, t1774: F, t1214: F, t5245: F, t3584: F, t16641: F, t16645: F, t16647: F, t16649: F, t16651: F, t16654: F, t16657: F, t16660: F, t16664: F, t16667: F, t16671: F, t16675: F, t16679: F, t16681: F, t16684: F, t16687: F, t16690: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t16751 = t1280 * t16750;
    let t16756 = t5284 * t3153;
    let t16757 = t16756 * t5465;
    let t16763 = t1811 * t3588 * t1287;
    let t16768 = t13133 * t1774;
    let t16771 = t5245 * t1214;
    let t16772 = t1280 * t16771;
    let t16775 = t1774 * t3584;
    let t16776 = t1280 * t16775;
    let t16781 = -t16641 + t16645 + t16647 - t16649 + t16651 - t16654 - t16657 - t16660 + t16664 + t16667 + t16671 - t16675 + t16679 - t16681 + t16684 - t16687 + t16690;
    (t16751, t16756, t16757, t16763, t16768, t16771, t16772, t16775, t16776, t16781)
}
