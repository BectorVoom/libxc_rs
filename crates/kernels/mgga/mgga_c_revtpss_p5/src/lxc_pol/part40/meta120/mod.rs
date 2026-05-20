//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta120 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk600;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk601;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk602;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk603;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk604;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta120<F: Float>(t72: F, t853: F, t245: F, t231: F, t775: F, t125: F, t836: F, t2722: F, t827: F, t828: F, t2695: F, t2702: F, t2704: F, t2707: F, t2716: F, t2721: F, t2726: F, t2730: F, t2732: F, t2739: F, t2742: F, t2745: F, t799: F, t825: F, t2479: F, t2488: F, t2648: F, t2653: F, t2656: F, t2666: F, t2672: F, t2678: F, t2686: F, t2691: F, t851: F) -> (F, F, F, F, F, F, F) {
        let (t2746, t2747) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk600::<F>(t72, t853, t245);
        let t2749 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk601::<F>(t231, t775);
        let (t2751, t2754) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk602::<F>(t125, t2749, t836, t2747, t231, t2722);
        let (t2756, t2759) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk603::<F>(t2754, t827, t828, t2695, t2702, t2704, t2707, t2716, t2721, t2726, t2730, t2732, t2739, t2742, t2745, t2751, t799, t825);
        let t2760 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk604::<F>(t2479, t2488, t2648, t2653, t2656, t2666, t2672, t2678, t2686, t2691, t2759, t825, t851);
    (t2746, t2747, t2749, t2751, t2754, t2756, t2760)
}
