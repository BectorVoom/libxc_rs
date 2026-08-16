//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta346 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1829;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1830;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1831;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1832;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1833;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta346<F: Float>(t11670: F, t3089: F, t1087: F, t3090: F, t3278: F, t3133: F, t73: F, t3153: F, t2258: F, t3094: F, t3182: F, t828: F, t2852: F, t357: F, t2251: F, t3109: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t11671 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1829::<F>(t11670, t3089);
        let t11672 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1830::<F>(t1087, t11671);
        let t11675 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1831::<F>(t3090, t3278);
        let (t11678, t11687, t11696, t11703) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1832::<F>(t3133, t73, t3153, t2258, t3094, t3182, t828);
        let (t11704, t11705, t11710) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1833::<F>(t2852, t357, t2251, t3109, t828);
    (t11671, t11672, t11675, t11678, t11687, t11696, t11703, t11704, t11705, t11710)
}
