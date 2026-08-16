//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta607 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2343;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2344;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2345;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2346;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2347;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2348;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta607(t268: f64, t9518: f64, t9540: f64, t681: f64, t702: f64, t793: f64, t215: f64, t2564: f64, t2567: f64, t2566: f64, t2576: f64, t9311: f64, t9313: f64, t2580: f64, t2583: f64, t130: f64, t39525: f64, t2563: f64) -> (f64, f64, f64, f64, f64, f64) {
        let t39786 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2343(t268, t9518, t9540);
        let t39791 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2344(t268, t681, t702, t793);
        let t39795 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2345(t215, t2564, t2567, t268);
        let t39799 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2346(t2566, t2576, t9311, t9313);
        let t39807 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2347(t2580, t2583, t130, t39525);
        let t39813 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2348(t130, t2563, t2580, t39525, t9313);
    (t39786, t39791, t39795, t39799, t39807, t39813)
}
