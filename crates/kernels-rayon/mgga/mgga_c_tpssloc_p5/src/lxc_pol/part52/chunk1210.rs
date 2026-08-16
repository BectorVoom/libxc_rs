//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1210/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1210(t32852: f64, t858: f64, t30640: f64, t32791: f64, t32794: f64, t32796: f64, t32800: f64, t32804: f64, t32811: f64, t32817: f64, t4147: f64, t4268: f64, t6627: f64, t7517: f64, t8353: f64, t8363: f64, t855: f64) -> (f64, f64) {
    let t32853 = t858 * t32852;
    let t32860 = -6.0_f64 * t32796 * t855 + 4.0_f64 * t32800 * t855 + 2.0_f64 * t32804 * t855 - t32853 * t855 + 2.0_f64 * t4147 * t8353 + 2.0_f64 * t4268 * t8353 - t4268 * t8363 + 4.0_f64 * t6627 * t7517 - t30640 - t32791 - t32794 + t32811 + t32817;
    (t32853, t32860)
}
