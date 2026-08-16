//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta342 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1814;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1815;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1816;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1817;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta342(t276: f64, t285: f64, t273: f64, t2439: f64, t931: f64, t2915: f64, t698: f64, t2922: f64, t913: f64, t275: f64, t290: f64, t2925: f64, t2935: f64, t945: f64, t2967: f64, t941: f64, t2966: f64, t307: f64, t302: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11354, t11358, t11366) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1814(t276, t285, t273, t2439, t931);
        let (t11368, t11384, t11385) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1815(t2915, t698, t2922, t913, t275);
        let (t11387, t11399, t11404) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1816(t290, t2925, t2935, t945, t2967, t941);
        let (t11408, t11409) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1817(t2966, t307, t302);
    (t11354, t11358, t11366, t11368, t11384, t11385, t11387, t11399, t11404, t11408, t11409)
}
