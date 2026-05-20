//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta105 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk720;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk721;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk722;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk723;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk724;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk725;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk726;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta105<F: Float>(t2566: F, t701: F, t2565: F, t2435: F, t2439: F, t2502: F, t2504: F, t2509: F, t2511: F, t682: F, t680: F, t130: F, t146: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2567, t2569) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk720::<F>(t2566, t701, t2565);
        let t2576 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk721::<F>(t2435, t2439, t2502, t2504, t2509, t2511);
        let (t2577, t2579) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk722::<F>(t2576, t701, t682);
        let t2580 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk723::<F>(t680);
        let (t2581, t2582) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk724::<F>(t2580, t130);
        let (t2583, t2584) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk725::<F>(t146);
        let (t2585, t2587) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk726::<F>(t2566, t2584, t2582);
    (t2567, t2569, t2576, t2577, t2579, t2580, t2581, t2582, t2583, t2584, t2585, t2587)
}
