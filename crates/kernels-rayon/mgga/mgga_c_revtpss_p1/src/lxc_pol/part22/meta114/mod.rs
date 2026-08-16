//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta114 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk771;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk772;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk773;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk774;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk775;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk776;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk777;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk778;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta114(t2754: f64, t827: f64, t828: f64, t2695: f64, t2702: f64, t2704: f64, t2707: f64, t2716: f64, t2721: f64, t2726: f64, t2730: f64, t2732: f64, t2739: f64, t2742: f64, t2745: f64, t2751: f64, t799: f64, t825: f64, t2479: f64, t2488: f64, t2648: f64, t2653: f64, t2656: f64, t2666: f64, t2672: f64, t2678: f64, t2686: f64, t2691: f64, t851: f64, t225: f64, t213: f64, t860: f64, t256: f64, t866: f64, t886: f64, t2435: f64, t871: f64, t785: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2756, t2759) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk771(t2754, t827, t828, t2695, t2702, t2704, t2707, t2716, t2721, t2726, t2730, t2732, t2739, t2742, t2745, t2751, t799, t825);
        let t2760 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk772(t2479, t2488, t2648, t2653, t2656, t2666, t2672, t2678, t2686, t2691, t2759, t825, t851);
        let (t2761, t2765) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk773(t225, t2760, t213, t860);
        let t2769 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk774(t256, t866);
        let t2770 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk775(t225, t2769);
        let t2771 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk776(t886);
        let t2772 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk777(t2770, t2771);
        let (t2776, t2777) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk778(t2435, t871, t225, t785);
    (t2756, t2760, t2761, t2765, t2769, t2770, t2771, t2772, t2776, t2777)
}
