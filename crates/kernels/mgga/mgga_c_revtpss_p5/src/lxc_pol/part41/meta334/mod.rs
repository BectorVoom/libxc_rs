//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta334 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1132;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1133;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta334<F: Float>(t11044: F, t4481: F, t2435: F, t4477: F, t136: F, t1579: F, t2457: F, t10504: F, t2471: F, t4325: F, t1580: F, t2444: F, t689: F, t213: F, t4469: F, t2440: F, t2439: F, t1569: F, t2453: F, t2458: F, t4321: F, t887: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t14995, t14998, t15004, t15006, t15008) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1132::<F>(t11044, t4481, t2435, t4477, t136, t1579, t2457, t10504, t2471, t4325, t1580, t2444);
        let (t15010, t15011, t15015, t15018, t15045) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1133::<F>(t15008, t689, t213, t4469, t1580, t2440, t2439, t1569, t2453, t2458, t4321, t887);
    (t14995, t14998, t15004, t15006, t15010, t15011, t15015, t15018, t15045)
}
