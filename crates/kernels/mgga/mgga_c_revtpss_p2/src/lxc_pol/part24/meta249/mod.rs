//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta249 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1012;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta249<F: Float>(t136: F, t1579: F, t2457: F, t10504: F, t2471: F, t4325: F, t1580: F, t2440: F, t2439: F, t1569: F, t2453: F, t2458: F) -> (F, F, F, F, F, F, F, F) {
        let (t15002, t15003, t15004, t15006, t15014, t15015, t15017, t15018) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1012::<F>(t136, t1579, t2457, t10504, t2471, t4325, t1580, t2440, t2439, t1569, t2453, t2458);
    (t15002, t15003, t15004, t15006, t15014, t15015, t15017, t15018)
}
