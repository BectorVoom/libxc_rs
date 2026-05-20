//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta99 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk671;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk672;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk673;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk674;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta99<F: Float>(t2453: F, t252: F, t136: F, t257: F, t124: F, t137: F, t68: F, t786: F, t861: F, t789: F, t867: F, t676: F, t886: F, t123: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2454, t2455, t2456, t2457) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk671::<F>(t2453, t252, t136, t257, t124, t137, t68);
        let t2458 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk672::<F>(t2455, t2457);
        let (t2460, t2461, t2462, t2464, t2465) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk673::<F>(t2454, t2458, t786, t861, t789, t252, t867);
        let (t2466, t2467) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk674::<F>(t676, t886, t123);
    (t2454, t2455, t2456, t2457, t2458, t2460, t2461, t2462, t2464, t2465, t2466, t2467)
}
