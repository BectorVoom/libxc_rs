//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta767 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2719;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2720;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta767(t50014: f64, t50033: f64, t162: f64, t187: f64, t40092: f64, t40094: f64, t14365: f64, t14397: f64, t2403: f64, t39818: f64, t39823: f64, t40084: f64, t40088: f64, t49992: f64, t49994: f64, t49995: f64, t14383: f64, t2398: f64, t40108: f64, t14616: f64, t2619: f64, t40207: f64, t4403: f64, t40119: f64, t40121: f64, t14386: f64, t2615: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50034, t50037, t50038, t50039, t50040) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2719(t50014, t50033, t162, t187, t40092, t40094, t14365, t14397, t2403, t39818, t39823, t40084, t40088, t49992, t49994, t49995);
        let (t50045, t50046, t50048, t50051, t50055, t50056, t50058) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2720(t14383, t2398, t40108, t14616, t2619, t162, t40207, t4403, t40119, t40121, t14386, t2615);
    (t50034, t50037, t50038, t50039, t50040, t50045, t50046, t50048, t50051, t50055, t50056, t50058)
}
