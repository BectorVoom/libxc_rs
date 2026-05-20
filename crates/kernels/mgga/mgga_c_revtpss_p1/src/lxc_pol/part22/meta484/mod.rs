//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta484 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2200;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2201;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2202;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta484<F: Float>(t11922: F, t4906: F, t3115: F, t15957: F, t4910: F, t3117: F, t3075: F, t357: F, t4781: F, t11670: F, t4890: F, t3317: F, t3299: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t16035, t16037, t16039, t16040, t16043, t16044, t16045, t16048) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2200::<F>(t11922, t4906, t3115, t15957, t4910, t3117, t3075, t357, t4781, t11670, t4890);
        let t16049 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2201::<F>(t16048, t3317);
        let t16052 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2202::<F>(t16048, t3299);
    (t16035, t16037, t16039, t16040, t16043, t16044, t16045, t16048, t16049, t16052)
}
