//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta106 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk650;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk651;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta106<F: Float>(t143: F, t680: F, t130: F, t700: F, t701: F, t2435: F, t2439: F, t2502: F, t2504: F, t2509: F, t2511: F, t682: F) -> (F, F, F, F, F, F, F, F) {
        let (t2564, t2565, t2566, t2567, t2569) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk650::<F>(t143, t680, t130, t700, t701);
        let (t2576, t2577, t2579) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk651::<F>(t2435, t2439, t2502, t2504, t2509, t2511, t701, t682);
    (t2564, t2565, t2566, t2567, t2569, t2576, t2577, t2579)
}
