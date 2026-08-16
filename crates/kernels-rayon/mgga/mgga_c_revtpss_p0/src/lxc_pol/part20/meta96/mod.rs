//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta96 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk551;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk552;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk553;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk554;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk555;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk556;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk557;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta96(t225: f64, t2735: f64, t826: f64, t849: f64, t820: f64, t823: f64, t843: f64, t839: f64, t241: f64, t72: f64, t853: f64, t245: f64, t231: f64, t775: f64, t125: f64, t836: f64, t2722: f64, t827: f64, t828: f64, t2695: f64, t2702: f64, t2704: f64, t2707: f64, t2716: f64, t2721: f64, t2726: f64, t2730: f64, t2732: f64, t799: f64, t825: f64, t2479: f64, t2488: f64, t2648: f64, t2653: f64, t2656: f64, t2666: f64, t2672: f64, t2678: f64, t2686: f64, t2691: f64, t851: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2736, t2737, t2739, t2741) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk551(t225, t2735, t826, t849, t820, t823, t843);
        let (t2742, t2745) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk552(t2741, t839, t241, t820, t823);
        let (t2746, t2747) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk553(t72, t853, t245);
        let t2749 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk554(t231, t775);
        let (t2751, t2754) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk555(t125, t2749, t836, t2747, t231, t2722);
        let (t2756, t2759) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk556(t2754, t827, t828, t2695, t2702, t2704, t2707, t2716, t2721, t2726, t2730, t2732, t2739, t2742, t2745, t2751, t799, t825);
        let t2760 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk557(t2479, t2488, t2648, t2653, t2656, t2666, t2672, t2678, t2686, t2691, t2759, t825, t851);
    (t2736, t2737, t2741, t2745, t2746, t2747, t2749, t2751, t2754, t2756, t2760)
}
