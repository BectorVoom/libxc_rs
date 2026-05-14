//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 856/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk856<F: Float>(t918: F, t974: F, t140: F, t191: F, t139: F, t969: F, t1003: F, t2933: F, t932: F, t132: F, t2934: F, t854: F, t60: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15193 = t918 * t974;
    let t15195 = t140 * t15193 * t191;
    let t15197 = t139 * t969;
    let t15198 = t15197 * t1003;
    let t15200 = t2933 * t932;
    let t15202 = 1.0 / t2934 / t132;
    let t15203 = t15200 * t15202;
    let t15206 = t854 * t854;
    let t15207 = 1.0 / t15206;
    let t15208 = t60 * t15207;
    (t15193, t15195, t15197, t15198, t15200, t15202, t15203, t15206, t15207, t15208)
}
