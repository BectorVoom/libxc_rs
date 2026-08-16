//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta338 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1653;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1654;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1655;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta338(t290: f64, t2925: f64, t11300: f64, t11385: f64, t3022: f64, t3030: f64, t3034: f64, t3006: f64, t3011: f64, t4733: f64, t981: f64, t2935: f64, t945: f64, t2967: f64, t941: f64, t2966: f64, t307: f64, t302: f64, t2944: f64, t953: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11387, t11388, t11390, t11392, t11394, t11396, t11398, t11399) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1653(t290, t2925, t11300, t11385, t3022, t3030, t3034, t3006, t3011, t4733, t981, t2935, t945);
        let (t11404, t11408, t11409) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1654(t2967, t941, t2966, t307, t302);
        let t11410 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1655(t2944, t953);
    (t11387, t11388, t11390, t11392, t11394, t11396, t11398, t11399, t11404, t11408, t11409, t11410)
}
