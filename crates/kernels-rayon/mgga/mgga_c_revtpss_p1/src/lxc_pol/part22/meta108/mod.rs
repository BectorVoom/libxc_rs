//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta108 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk738;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk739;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk740;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk741;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk742;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk743;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk744;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta108(t123: f64, t192: f64, t676: f64, t762: f64, t2392: f64, t2400: f64, t2402: f64, t2416: f64, t2498: f64, t2518: f64, t2522: f64, t2525: f64, t2527: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t2610: f64, t2614: f64, t2617: f64, t2621: f64, t2624: f64, t2628: f64, t225: f64, t73: f64, t853: f64, t2394: f64, t2430: f64, t832: f64, t227: f64, t229: f64, t830: f64, t833: f64, t231: f64, t827: f64, t828: f64, t820: f64, t843: f64, t849: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2629 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk738(t123, t192);
        let t2630 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk739(t676, t762);
        let t2632 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk740(t2629, t2630);
        let t2633 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk741(t2392, t2400, t2402, t2416, t2498, t2518, t2522, t2525, t2527, t2562, t2569, t2579, t2587, t2610, t2614, t2617, t2621, t2624, t2628, t2632);
        let (t2634, t2638, t2639, t2642, t2645) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk742(t225, t2633, t73, t853, t2394, t2430, t832, t227, t229, t830, t833);
        let t2646 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk743(t231, t2645);
        let (t2648, t2652) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk744(t2646, t827, t828, t820, t843, t849);
    (t2629, t2630, t2632, t2634, t2638, t2639, t2642, t2645, t2646, t2648, t2652)
}
