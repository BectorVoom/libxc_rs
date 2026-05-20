//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2978/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2978<F: Float>(t10142: F, t14113: F, t49180: F, t10136: F, t14239: F, t10119: F, t4101: F, t5740: F, t9288: F, t1419: F, t5658: F, t2782: F, t4086: F, t543: F) -> (F, F, F, F, F) {
    let t49189 = t49180 * t14113 * t10142;
    let t49198 = t14239 * t10136;
    let t49200 = t14239 * t10119;
    let t49203 = t4101 * t5740 * t9288;
    let t49205 = t1419 * t5658;
    let t49208 = t2782 * t4086 * t49205 * t543;
    (t49189, t49198, t49200, t49203, t49208)
}
