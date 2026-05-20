//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3082/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3082<F: Float>(t11144: F, t53321: F, t11970: F, t1660: F, t27527: F, t2852: F, t11150: F, t27531: F, t15817: F, t3173: F, t16158: F, t3188: F) -> (F, F, F, F, F, F) {
    let t53322 = t53321 * t11144;
    let t53326 = t1660 * t11970;
    let t53328 = t27527 * t2852;
    let t53332 = t27531 * t11150;
    let t53353 = t15817 * t3173;
    let t53359 = t3188 * t16158;
    (t53322, t53326, t53328, t53332, t53353, t53359)
}
