//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2928/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2928<F: Float>(t10175: F, t14090: F, t14100: F, t9671: F, t1357: F, t14269: F, t689: F, t1358: F, t14066: F, t212: F, t13746: F, t686: F, t72: F, t9680: F) -> (F, F, F, F, F) {
    let t47813 = t10175 * t14090;
    let t47816 = t14100 * t9671;
    let t47819 = t689 * t1357 * t14269;
    let t47825 = t689 * t212 * t14066 * t1358;
    let t47832 = t9680 * t13746 * t72 * t686;
    (t47813, t47816, t47819, t47825, t47832)
}
