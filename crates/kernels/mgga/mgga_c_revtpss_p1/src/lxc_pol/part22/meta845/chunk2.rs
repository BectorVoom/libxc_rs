//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2982/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2982<F: Float>(t10069: F, t14225: F, t10013: F, t14224: F, t2782: F, t48073: F, t543: F, t4100: F, t4086: F, t49213: F, t10136: F, t14114: F) -> (F, F, F, F, F) {
    let t49289 = t10069 * t14225;
    let t49296 = t2782 * t10013 * t14224;
    let t49306 = t48073 * t543;
    let t49308 = t2782 * t4100 * t49306;
    let t49313 = t2782 * t4086 * t49213 * t543;
    let t49321 = t14114 * t10136;
    (t49289, t49296, t49308, t49313, t49321)
}
