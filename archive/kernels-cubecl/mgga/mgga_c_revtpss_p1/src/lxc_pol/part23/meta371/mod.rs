//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta371 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1700;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta371<F: Float>(t3172: F, t4874: F, t3127: F, t4802: F, t1063: F, t4807: F, t3153: F, t4866: F, t11922: F, t4911: F, t3115: F, t1032: F, t4743: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t15769, t15771, t15772, t15774, t15775, t15776, t15780, t15794, t15796, t15816) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1700::<F>(t3172, t4874, t3127, t4802, t1063, t4807, t3153, t4866, t11922, t4911, t3115, t1032, t4743);
    (t15769, t15771, t15772, t15774, t15775, t15776, t15780, t15794, t15796, t15816)
}
