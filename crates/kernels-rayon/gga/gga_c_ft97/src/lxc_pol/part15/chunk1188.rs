//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1188/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1188(t10580: f64, t10603: f64, t10613: f64, t14961: f64, t2766: f64, t2771: f64, t4206: f64, t43834: f64, t43913: f64, t43918: f64, t462: f64, t70999: f64, t848: f64, t88253: f64, t88257: f64, t88261: f64, t88273: f64, t88606: f64, t88612: f64, t89779: f64, t89822: f64, t89826: f64, t89870: f64, t89877: f64, t89881: f64, t89885: f64) -> f64 {
    let t90421 = 40.0_f64 / 9.0_f64 * t462 * t10580 * t88253 - 8.0_f64 * t462 * t2766 * t88273 + 8.0_f64 * t462 * t848 * t88257 + 2.0_f64 * t462 * t848 * t88261 + 16.0_f64 / 9.0_f64 * t70999 - 8.0_f64 / 3.0_f64 * t462 * t43918 * t89881 - 12.0_f64 * t462 * t4206 * t88606 - 20.0_f64 / 9.0_f64 * t462 * t14961 * t88612 - 4.0_f64 * t462 * t2771 * t89885 + 4.0_f64 / 3.0_f64 * t462 * t10613 * t89877 - 4.0_f64 * t462 * t10603 * t89822 + 4.0_f64 / 3.0_f64 * t462 * t2771 * t89826 + 8.0_f64 * t462 * t43913 * t89779 + 40.0_f64 / 27.0_f64 * t462 * t43834 * t89870;
    t90421
}
