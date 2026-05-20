//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta360 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1674;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta360<F: Float>(t15002: F, t2457: F, t10504: F, t2471: F, t4325: F, t1580: F, t2444: F, t689: F, t213: F, t4469: F, t2440: F, t2439: F) -> (F, F, F, F, F, F, F, F) {
        let (t15003, t15004, t15006, t15008, t15010, t15011, t15014, t15015) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1674::<F>(t15002, t2457, t10504, t2471, t4325, t1580, t2444, t689, t213, t4469, t2440, t2439);
    (t15003, t15004, t15006, t15008, t15010, t15011, t15014, t15015)
}
