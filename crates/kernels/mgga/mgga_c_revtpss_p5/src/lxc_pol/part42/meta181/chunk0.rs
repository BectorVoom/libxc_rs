//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 754/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk754<F: Float>(t1043: F, t357: F, t4893: F, t3117: F, t1651: F, t1045: F, t999: F, t4781: F, t1012: F, t1014: F, t4579: F, t3252: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4900 = t1043 * t357;
    let t4901 = t4893 * t4900;
    let t4902 = t3117 * t4901;
    let t4905 = t1651 * t1043;
    let t4906 = t4905 * t1045;
    let t4907 = t3117 * t4906;
    let t4910 = t357 * t999;
    let t4911 = t4781 * t4910;
    let t4912 = t3117 * t4911;
    let t4915 = t1012 * t1014;
    let t4916 = t4915 * t4579;
    let t4919 = t1012 * t3252;
    (t4900, t4901, t4902, t4905, t4906, t4907, t4910, t4911, t4912, t4915, t4916, t4919)
}
