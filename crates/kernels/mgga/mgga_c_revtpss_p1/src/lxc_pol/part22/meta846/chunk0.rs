//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2983/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2983<F: Float>(t1882: F, t2482: F, t4104: F, t4118: F, t1398: F, t2782: F, t4086: F, t543: F, t5710: F, t1897: F, t40317: F, t10111: F, t22: F, t5759: F) -> (F, F, F, F) {
    let t49325 = t2482 * t4118 * t1882 * t4104;
    let t49346 = t2782 * t4086 * t5710 * t1398 * t543;
    let t49354 = t40317 * t1897;
    let t49361 = t10111 * t5759 * t22;
    (t49325, t49346, t49354, t49361)
}
