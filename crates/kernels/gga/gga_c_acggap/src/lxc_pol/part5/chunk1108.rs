//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1108/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1108<F: Float>(t13804: F, t13805: F, t16209: F, t16211: F, t16213: F, t21669: F, t21671: F, t21675: F, t21679: F, t21681: F, t21684: F, t21687: F, t21691: F, t21695: F, t13696: F, t13699: F, t13701: F, t13706: F, t13714: F, t13729: F, t13810: F, t13812: F, t16230: F, t21707: F, t21709: F, t21712: F, t21714: F, t21717: F) -> (F, F) {
    let t22563 = 4.0 * t21669 - 8.0 / 3.0 * t21671 - 3.0 / 2.0 * t21675 + t21679 - 4.0 / 3.0 * t21681 + t21684 + t21687 / 2.0 + 6.0 * t21691 - 3.0 / 2.0 * t21695 + 2.0 / 3.0 * t16209 - 28.0 / 9.0 * t16211 + 140.0 / 27.0 * t16213 - t13804 + t13805;
    let t22575 = -40.0 / 27.0 * t13696 + 4.0 / 3.0 * t13699 + t13701 / 6.0 + t13706 / 6.0 - t13714 / 12.0 + t13810 - t13729 / 3.0 + t13812 + 2.0 / 3.0 * t21707 + 14.0 / 9.0 * t21709 + t21712 - 7.0 / 9.0 * t21714 - t21717 / 4.0 + 2.0 / 3.0 * t16230;
    (t22563, t22575)
}
