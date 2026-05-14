//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1308/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1308<F: Float>(t20053: F, t5601: F, t5874: F, t1163: F, t26992: F, t6183: F, t1339: F, t32000: F, t8176: F, t26856: F, t32045: F, t3759: F, t25370: F, t9461: F, t1328: F, t8054: F) -> (F, F, F, F, F, F) {
    let t118781 = t20053 * t5874 * t5601;
    let t118785 = t6183 * t26992 * t1163;
    let t118789 = t1339 * t32000 * t8176;
    let t118792 = t3759 * t32045 * t26856;
    let t118795 = t3759 * t9461 * t25370;
    let t118797 = t1328 * t8054;
    (t118781, t118785, t118789, t118792, t118795, t118797)
}
