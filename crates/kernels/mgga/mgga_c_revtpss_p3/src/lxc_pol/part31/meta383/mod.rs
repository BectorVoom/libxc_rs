//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta383 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1420;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1421;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta383<F: Float>(t3173: F, t4879: F, t4866: F, t73: F, t11710: F, t4782: F, t3091: F, t1014: F, t140: F, t4579: F, t1011: F, t3252: F, t4574: F, t1012: F, t11821: F, t11922: F, t4906: F, t3115: F, t4895: F, t4892: F, t4886: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15942, t15957, t15984, t15986, t15990, t15993) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1420::<F>(t3173, t4879, t4866, t73, t11710, t4782, t3091, t1014, t140, t4579, t1011, t3252);
        let (t15996, t16012, t16035, t16037, t16055, t16057, t16060) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1421::<F>(t15993, t4574, t1011, t1012, t11821, t11922, t4906, t3115, t4895, t4892, t140, t4886);
    (t15942, t15957, t15984, t15986, t15990, t15996, t16012, t16035, t16037, t16055, t16057, t16060)
}
