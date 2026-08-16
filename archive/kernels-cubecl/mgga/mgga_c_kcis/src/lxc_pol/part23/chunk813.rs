//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 813/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk813<F: Float>(t1464: F, t15812: F, t3954: F, t5632: F, t1468: F, t4124: F, t4123: F, t3734: F, t5633: F, t11881: F, t1948: F, t2046: F, t3805: F) -> (F, F, F, F, F, F, F, F) {
    let t15813 = t1464 * t15812;
    let t15815 = t5632 * t3954;
    let t15816 = t1468 * t15815;
    let t15817 = t1464 * t15816;
    let t15819 = t5632 * t4124;
    let t15820 = t4123 * t15819;
    let t15821 = t1464 * t15820;
    let t15823 = t3734 * t5633;
    let t15824 = t1464 * t15823;
    let t15826 = t11881 * t1948;
    let t15828 = t2046 * t3805;
    (t15813, t15815, t15817, t15819, t15821, t15824, t15826, t15828)
}
