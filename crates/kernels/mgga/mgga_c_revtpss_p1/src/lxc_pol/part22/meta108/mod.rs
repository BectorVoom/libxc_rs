//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta108 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk738;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk739;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk740;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk741;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk742;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk743;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk744;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta108<F: Float>(t123: F, t192: F, t676: F, t762: F, t2392: F, t2400: F, t2402: F, t2416: F, t2498: F, t2518: F, t2522: F, t2525: F, t2527: F, t2562: F, t2569: F, t2579: F, t2587: F, t2610: F, t2614: F, t2617: F, t2621: F, t2624: F, t2628: F, t225: F, t73: F, t853: F, t2394: F, t2430: F, t832: F, t227: F, t229: F, t830: F, t833: F, t231: F, t827: F, t828: F, t820: F, t843: F, t849: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t2629 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk738::<F>(t123, t192);
        let t2630 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk739::<F>(t676, t762);
        let t2632 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk740::<F>(t2629, t2630);
        let t2633 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk741::<F>(t2392, t2400, t2402, t2416, t2498, t2518, t2522, t2525, t2527, t2562, t2569, t2579, t2587, t2610, t2614, t2617, t2621, t2624, t2628, t2632);
        let (t2634, t2638, t2639, t2642, t2645) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk742::<F>(t225, t2633, t73, t853, t2394, t2430, t832, t227, t229, t830, t833);
        let t2646 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk743::<F>(t231, t2645);
        let (t2648, t2652) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk744::<F>(t2646, t827, t828, t820, t843, t849);
    (t2629, t2630, t2632, t2634, t2638, t2639, t2642, t2645, t2646, t2648, t2652)
}
