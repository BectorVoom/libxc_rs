//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta109 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk745;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk746;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk747;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk748;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk749;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk750;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta109<F: Float>(t2652: F, t857: F, t2430: F, t828: F, t855: F, t212: F, t27: F, t225: F, t816: F, t240: F, t823: F, t243: F, t836: F, t231: F, t596: F, t813: F, t2482: F, t849: F, t136: F, t854: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2653, t2656, t2659) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk745::<F>(t2652, t857, t2430, t828, t855, t212, t27);
        let t2661 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk746::<F>(t225, t2659, t816);
        let t2662 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk747::<F>(t240, t823);
        let (t2664, t2665, t2666, t2668) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk748::<F>(t243, t836, t231, t2662, t2661, t240, t596);
        let (t2672, t2674) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk749::<F>(t243, t2668, t816, t813, t2482, t27, t849);
        let t2675 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk750::<F>(t136, t854);
    (t2653, t2656, t2659, t2661, t2662, t2664, t2665, t2666, t2668, t2672, t2674, t2675)
}
