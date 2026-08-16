//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1068/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1068(t2: f64, t42123: f64, t1775: f64, t9913: f64, t9928: f64, t9910: f64, t2493: f64, t3910: f64, t3917: f64, t41464: f64, t41490: f64, t41827: f64, t41833: f64, t41880: f64, t41884: f64, t42105: f64, t42107: f64, t42110: f64, t42117: f64, t42119: f64, t42121: f64, t462: f64, t9916: f64) -> f64 {
    let t42124 = t42123 * t2;
    let t42131 = t1775 * t9913;
    let t42133 = t1775 * t9928;
    let t42141 = t1775 * t9910;
    let t42143 = -8.0_f64 * t42105 + 16.0_f64 / 9.0_f64 * t42107 - 8.0_f64 / 3.0_f64 * t462 * t42110 * t41880 - 4.0_f64 * t462 * t2493 * t41833 - 4.0_f64 / 3.0_f64 * t42117 - 8.0_f64 / 9.0_f64 * t42119 + 8.0_f64 / 3.0_f64 * t42121 + 8.0_f64 * t462 * t42124 * t41827 - 16.0_f64 / 3.0_f64 * t462 * t9916 * t41884 - 8.0_f64 / 3.0_f64 * t42131 - 4.0_f64 / 3.0_f64 * t42133 + 8.0_f64 / 3.0_f64 * t462 * t3917 * t41490 + 8.0_f64 * t462 * t3910 * t41464 + 8.0_f64 / 3.0_f64 * t42141;
    t42143
}
