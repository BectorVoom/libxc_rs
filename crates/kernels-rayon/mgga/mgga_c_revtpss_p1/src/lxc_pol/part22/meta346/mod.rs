//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta346 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1829;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1830;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1831;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1832;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1833;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta346(t11670: f64, t3089: f64, t1087: f64, t3090: f64, t3278: f64, t3133: f64, t73: f64, t3153: f64, t2258: f64, t3094: f64, t3182: f64, t828: f64, t2852: f64, t357: f64, t2251: f64, t3109: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t11671 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1829(t11670, t3089);
        let t11672 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1830(t1087, t11671);
        let t11675 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1831(t3090, t3278);
        let (t11678, t11687, t11696, t11703) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1832(t3133, t73, t3153, t2258, t3094, t3182, t828);
        let (t11704, t11705, t11710) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1833(t2852, t357, t2251, t3109, t828);
    (t11671, t11672, t11675, t11678, t11687, t11696, t11703, t11704, t11705, t11710)
}
