//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 732/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk732(t20759: f64, t2221: f64, t1053: f64, t4431: f64, t2211: f64, t2210: f64, t20224: f64, t3434: f64, t3491: f64, t4778: f64, t91: f64, t13119: f64, t13123: f64, t17214: f64, t17249: f64, t17250: f64, t17251: f64, t20536: f64, t20540: f64, t20551: f64, t20666: f64, t20669: f64, t9383: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20760 = t2221 * t20759;
    let t20763 = t4431 * t1053;
    let t20764 = t2211 * t20763;
    let t20765 = t2210 * t20764;
    let t20768 = t3434 * t20224;
    let t20769 = t2210 * t20768;
    let t20779 = t91 * t3491 * t4778;
    let t20781 = -t17214 + 2.0_f64 * t20666 - t20669 / 9.0_f64 - t13119 - t9383 - t13123 - 10.0_f64 / 81.0_f64 * t20536 - 2.0_f64 / 3.0_f64 * t20540 + 4.0_f64 / 9.0_f64 * t20551 - t20779 / 4.0_f64 + t17249 - t17250 + t17251;
    (t20760, t20763, t20764, t20765, t20768, t20769, t20779, t20781)
}
