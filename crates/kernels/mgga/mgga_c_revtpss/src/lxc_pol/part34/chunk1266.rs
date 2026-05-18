//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1266/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1266<F: Float>(t1925: F, t2247: F, t5819: F, t1469: F, t603: F, t13272: F, t28126: F, t29524: F, t38: F, t1927: F, t5816: F, t1926: F) -> (F, F, F, F, F) {
    let t108753 = t2247 * t5819 * t1925;
    let t108757 = t603 * t1469 * t1925;
    let t108772 = t13272 * t28126;
    let t108782 = t2247 * t38 * t29524;
    let t108879 = t1927 * t5816;
    let t108880 = t1926 * t108879;
    (t108753, t108757, t108772, t108782, t108880)
}
