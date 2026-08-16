//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 926/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk926(t1882: f64, t8396: f64, t488: f64, t8326: f64, t8583: f64, t8574: f64, t110: f64, t1643: f64, t1820: f64, t1866: f64, t1876: f64, t1901: f64, t1904: f64, t358: f64, t379: f64, t38086: f64, t38654: f64, t39150: f64, t39154: f64, t446: f64, t447: f64, t499: f64, t7959: f64, t8219: f64, t83: f64, t8553: f64, t8557: f64) -> f64 {
    let t39161 = t1882 * t8396;
    let t39167 = t8326 * t488;
    let t39183 = t1882 * t8583;
    let t39185 = t1882 * t8574;
    let t39187 = -8.0_f64 / 3.0_f64 * t1901 * t39150 * t8219 + 8.0_f64 / 9.0_f64 * t39154 - 4.0_f64 / 3.0_f64 * t1901 * t8557 * t1820 * t358 * t1904 + 4.0_f64 / 3.0_f64 * t39161 - 4.0_f64 / 3.0_f64 * t1901 * t8557 * t8553 * t379 - 8.0_f64 / 9.0_f64 * t1901 * t39167 * t1876 * t1643 + 8.0_f64 * t446 * t83 * t38654 + 8.0_f64 / 3.0_f64 * t446 * t447 * t110 * t38086 + 16.0_f64 / 9.0_f64 * t446 * t1866 * t499 * t7959 + 4.0_f64 / 9.0_f64 * t39183 + 4.0_f64 / 27.0_f64 * t39185;
    t39187
}
