//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2404/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2404<F: Float>(t11003: F, t9303: F, t10978: F, t689: F, t779: F, t10981: F, t22: F, t868: F, t886: F, t10910: F, t212: F, t780: F) -> (F, F, F, F) {
    let t40970 = t9303 * t11003;
    let t40973 = t689 * t779 * t10978;
    let t40978 = t10981 * t868 * t22 * t886;
    let t40982 = t689 * t212 * t10910 * t780;
    (t40970, t40973, t40978, t40982)
}
