//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2962/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2962<F: Float>(t40690: F, t5610: F, t5618: F, t9784: F, t820: F, t844: F, t9991: F, t13807: F, t13767: F, t2661: F, t3829: F, t48347: F) -> (F, F, F, F, F) {
    let t48829 = t40690 * t5610;
    let t48833 = t9784 * t5618;
    let t48836 = t820 * t9991 * t844;
    let t48837 = t48836 * t13807;
    let t48845 = t2661 * t13767 * t48347 * t3829;
    (t48829, t48833, t48836, t48837, t48845)
}
