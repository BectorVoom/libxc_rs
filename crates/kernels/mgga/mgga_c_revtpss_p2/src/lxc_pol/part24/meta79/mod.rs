//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta79 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk474;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk475;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk476;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk477;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk478;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta79<F: Float>(t2455: F, t2457: F, t2454: F, t252: F, t867: F, t786: F, t215: F, t685: F, t788: F, t787: F, t206: F, t242: F, t240: F, t72: F, t225: F, t27: F, t823: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2458, t2460, t2464, t2465) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk474::<F>(t2455, t2457, t2454, t252, t867, t786);
        let t2470 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk475::<F>(t215, t685);
        let (t2471, t2473, t2475) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk476::<F>(t2470, t788, t787, t206, t242);
        let (t2476, t2477, t2482) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk477::<F>(t240, t2475, t72, t225, t786);
        let t2484 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk478::<F>(t2482, t27, t823);
    (t2458, t2460, t2464, t2465, t2470, t2471, t2473, t2475, t2476, t2477, t2482, t2484)
}
