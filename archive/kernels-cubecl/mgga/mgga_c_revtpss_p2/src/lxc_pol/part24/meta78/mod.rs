//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta78 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk471;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk472;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk473;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta78<F: Float>(t251: F, t785: F, t780: F, t2439: F, t211: F, t784: F, t209: F, t252: F, t136: F, t257: F, t124: F, t137: F, t68: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t2440, t2441, t2443, t2452) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk471::<F>(t251, t785, t780, t2439, t211, t784);
        let t2453 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk472::<F>(t209, t2452);
        let (t2454, t2455, t2456, t2457) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk473::<F>(t2453, t252, t136, t257, t124, t137, t68);
    (t2440, t2441, t2443, t2452, t2453, t2454, t2455, t2456, t2457)
}
