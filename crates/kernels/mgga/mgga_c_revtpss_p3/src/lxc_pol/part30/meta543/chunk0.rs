//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1975/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1975<F: Float>(t7575: F, t7719: F, t2122: F, t28089: F, t28150: F, t1923: F, t2123: F, t25162: F, t26792: F, t26795: F, t28093: F, t28147: F, t28154: F, t29364: F, t29367: F, t6954: F, t6963: F, t7576: F, t7579: F, t7702: F, t8144: F, t8147: F) -> (F, F, F, F) {
    let t29372 = t7575 * t7719;
    let t29375 = t2122 * t28089;
    let t29380 = t2122 * t28150;
    let t29387 = -t28093 * t2123 / F::new(6.0) - t7702 * t7576 / F::new(6.0) - t7702 * t7579 / F::new(6.0) - t6954 * t8144 / F::new(6.0) - t1923 * t29364 / F::new(6.0) - t1923 * t29367 / F::new(6.0) - t6954 * t8147 / F::new(6.0) - t1923 * t29372 / F::new(6.0) - t1923 * t29375 / F::new(6.0) - F::new(5.0) * t26792 * t28147 - F::new(5.0) / F::new(3.0) * t25162 * t29380 - F::new(5.0) / F::new(3.0) * t28154 * t26795 + t6963 * t8147 / F::new(3.0);
    (t29372, t29375, t29380, t29387)
}
