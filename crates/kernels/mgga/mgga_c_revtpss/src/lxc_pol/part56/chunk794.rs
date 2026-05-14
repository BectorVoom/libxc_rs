//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 794/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk794<F: Float>(t2122: F, t28150: F, t1923: F, t2123: F, t25162: F, t26792: F, t26795: F, t28093: F, t28147: F, t28154: F, t29364: F, t29367: F, t29372: F, t29375: F, t6954: F, t6963: F, t7576: F, t7579: F, t7702: F, t8144: F, t8147: F) -> (F,) {
    let t29380 = t2122 * t28150;
    let t29387 = -t28093 * t2123 / 6.0 - t7702 * t7576 / 6.0 - t7702 * t7579 / 6.0 - t6954 * t8144 / 6.0 - t1923 * t29364 / 6.0 - t1923 * t29367 / 6.0 - t6954 * t8147 / 6.0 - t1923 * t29372 / 6.0 - t1923 * t29375 / 6.0 - 5.0 * t26792 * t28147 - 5.0 / 3.0 * t25162 * t29380 - 5.0 / 3.0 * t28154 * t26795 + t6963 * t8147 / 3.0;
    (t29387,)
}
