//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3056/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3056<F: Float>(t14477: F, t2435: F, t14978: F, t2465: F, t686: F, t72: F, t14480: F, t252: F, t2782: F, t2828: F, t10073: F, t14482: F) -> (F, F, F, F) {
    let t51741 = t2435 * t14477;
    let t51746 = t2465 * t14978 * t72 * t686;
    let t51750 = t2782 * t252 * t14480 * t2828;
    let t51756 = t10073 * t14482;
    (t51741, t51746, t51750, t51756)
}
