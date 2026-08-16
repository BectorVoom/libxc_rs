//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta248 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1429;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1430;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1431;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1432;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta248(t745: f64, t9385: f64, t9368: f64, t2514: f64, t746: f64, t2495: f64, t744: f64, t2576: f64, t2582: f64, t2584: f64, t700: f64, t2519: f64, t2577: f64, t268: f64, t2581: f64, t675: f64, t2585: f64, t2565: f64, t702: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9485, t9488, t9501, t9507, t9508, t9514) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1429(t745, t9385, t9368, t2514, t746, t2495, t744, t2576, t2582, t2584, t700);
        let t9517 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1430(t2519, t2577, t268);
        let (t9518, t9521) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1431(t2581, t675, t2585, t268);
        let t9524 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1432(t2565, t2576, t702);
    (t9485, t9488, t9501, t9507, t9508, t9514, t9517, t9518, t9521, t9524)
}
