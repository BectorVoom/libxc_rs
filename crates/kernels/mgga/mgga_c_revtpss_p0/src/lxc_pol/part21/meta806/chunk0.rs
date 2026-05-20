//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2934/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2934<F: Float>(t11670: F, t15687: F, t3317: F, t127: F, t15690: F, t15689: F, t15692: F, t11916: F, t15932: F, t11922: F, t11927: F, t16026: F) -> (F, F, F, F, F, F) {
    let t53401 = t11670 * t15687;
    let t53402 = t3317 * t53401;
    let t53405 = t127 * t15690;
    let t53407 = t15689 * t53405 * t15692;
    let t53413 = t15932 * t11916;
    let t53416 = t11927 * t11922 * t16026;
    (t53401, t53402, t53405, t53407, t53413, t53416)
}
