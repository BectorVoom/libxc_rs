//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta373 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1919;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1920;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1921;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1922;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta373(t247: f64, t3368: f64, t3634: f64, t1261: f64, t3636: f64, t3647: f64, t3367: f64, t414: f64, t66: f64, t11239: f64, t1243: f64, t460: f64, t3727: f64, t473: f64, t3596: f64, t13038: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13089, t13090, t13092, t13099) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1919(t247, t3368, t3634, t1261, t3636, t3647, t3367, t414);
        let (t13100, t13126) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1920(t13099, t66, t11239, t1243);
        let (t13127, t13133, t13141) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1921(t13126, t460, t3727, t473, t11239, t3596);
        let (t13142, t13147) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1922(t13141, t460, t11239, t13038);
    (t13089, t13090, t13092, t13099, t13100, t13126, t13127, t13133, t13141, t13142, t13147)
}
