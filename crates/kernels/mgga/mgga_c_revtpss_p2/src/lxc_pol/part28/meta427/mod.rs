//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta427 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1608;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1609;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1610;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta427<F: Float>(t11922: F, t4906: F, t3115: F, t15957: F, t4910: F, t3117: F, t3075: F, t357: F, t4781: F, t11670: F, t4890: F, t3317: F, t3299: F, t4895: F, t4892: F, t140: F, t4886: F, t1011: F, t3241: F, t4924: F, t12047: F, t15905: F, t3151: F, t15907: F, t11883: F, t11888: F, t1656: F, t4887: F, t4896: F, t4902: F) -> (F, F, F, F, F, F, F) {
        let (t16035, t16037, t16040, t16045, t16048, t16049) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1608::<F>(t11922, t4906, t3115, t15957, t4910, t3117, t3075, t357, t4781, t11670, t4890, t3317);
        let (t16052, t16055, t16057, t16060, t16062, t16064, t16067) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1609::<F>(t16048, t3299, t11922, t4895, t4892, t140, t4886, t1011, t3241, t4924, t12047, t15905);
        let (t16070, t16073) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1610::<F>(t3151, t357, t15907, t3117, t11883, t11888, t16037, t16040, t16045, t16049, t16052, t16057, t16062, t16064, t16067, t1656, t3115, t3241, t4887, t4896, t4902);
    (t16035, t16040, t16045, t16055, t16060, t16070, t16073)
}
