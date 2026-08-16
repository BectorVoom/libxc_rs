//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta118 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk680;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk681;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk682;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk683;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk684;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk685;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta118<F: Float>(t225: F, t2633: F, t73: F, t853: F, t2394: F, t2430: F, t832: F, t227: F, t229: F, t830: F, t833: F, t231: F, t827: F, t828: F, t820: F, t843: F, t849: F, t857: F, t855: F, t212: F, t27: F, t816: F, t240: F, t823: F, t243: F, t836: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2634, t2639, t2642, t2645) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk680::<F>(t225, t2633, t73, t853, t2394, t2430, t832, t227, t229, t830, t833);
        let t2646 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk681::<F>(t231, t2645);
        let (t2648, t2652) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk682::<F>(t2646, t827, t828, t820, t843, t849);
        let (t2653, t2656, t2661) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk683::<F>(t2652, t857, t2430, t828, t855, t212, t27, t225, t816);
        let t2662 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk684::<F>(t240, t823);
        let t2664 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk685::<F>(t243, t836, t231);
    (t2634, t2639, t2642, t2645, t2646, t2648, t2652, t2653, t2656, t2661, t2662, t2664)
}
