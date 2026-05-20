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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk771;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk772;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk773;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk774;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk775;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk776;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk777;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk778;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta114<F: Float>(t2754: F, t827: F, t828: F, t2695: F, t2702: F, t2704: F, t2707: F, t2716: F, t2721: F, t2726: F, t2730: F, t2732: F, t2739: F, t2742: F, t2745: F, t2751: F, t799: F, t825: F, t2479: F, t2488: F, t2648: F, t2653: F, t2656: F, t2666: F, t2672: F, t2678: F, t2686: F, t2691: F, t851: F, t225: F, t213: F, t860: F, t256: F, t866: F, t886: F, t2435: F, t871: F, t785: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t2756, t2759) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk771::<F>(t2754, t827, t828, t2695, t2702, t2704, t2707, t2716, t2721, t2726, t2730, t2732, t2739, t2742, t2745, t2751, t799, t825);
        let t2760 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk772::<F>(t2479, t2488, t2648, t2653, t2656, t2666, t2672, t2678, t2686, t2691, t2759, t825, t851);
        let (t2761, t2765) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk773::<F>(t225, t2760, t213, t860);
        let t2769 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk774::<F>(t256, t866);
        let t2770 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk775::<F>(t225, t2769);
        let t2771 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk776::<F>(t886);
        let t2772 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk777::<F>(t2770, t2771);
        let (t2776, t2777) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk778::<F>(t2435, t871, t225, t785);
    (t2756, t2760, t2761, t2765, t2769, t2770, t2771, t2772, t2776, t2777)
}
