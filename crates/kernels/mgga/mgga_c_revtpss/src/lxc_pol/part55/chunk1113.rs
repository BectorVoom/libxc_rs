//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1113/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1113<F: Float>(t128594: F, t128609: F, t128625: F, t128647: F, t128664: F, t128677: F, t128688: F, t128713: F, t128742: F, t128767: F, t128781: F, t128796: F, t128810: F, t128826: F, t128837: F, t128860: F, t1450: F, t2014: F, t532: F) -> (F,) {
    let t128867 = t2014 * t532 * (t128594 + t128609 + t128625 + t128647 + t128664 + t128677 + t128688 + t128713 + t128742 + t128767 + t128781 + t128796 + t128810 + t128826 + t128837 + t128860) * t1450;
    (t128867,)
}
