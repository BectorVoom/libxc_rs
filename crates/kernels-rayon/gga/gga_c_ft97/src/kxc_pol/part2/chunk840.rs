//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 840/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk840(t13180: f64, t144: f64, t13084: f64, t13085: f64, t13090: f64, t13137: f64, t13143: f64, t13148: f64, t13152: f64, t13154: f64, t13157: f64, t13162: f64, t13168: f64, t13173: f64, t13177: f64, t1901: f64, t3281: f64, t446: f64, t9405: f64) -> f64 {
    let t13181 = t144 * t13180;
    let t13184 = t13084 - 2.0_f64 * t446 * t13085 - 2.0_f64 / 3.0_f64 * t446 * t13090 - t446 * t13137 / 3.0_f64 - 4.0_f64 / 3.0_f64 * t1901 * t13143 + 2.0_f64 / 9.0_f64 * t1901 * t13148 - t13152 + 2.0_f64 / 9.0_f64 * t1901 * t13154 + 2.0_f64 / 9.0_f64 * t1901 * t13157 + 2.0_f64 / 9.0_f64 * t1901 * t13162 - 2.0_f64 / 9.0_f64 * t1901 * t13168 + 2.0_f64 / 27.0_f64 * t9405 - 4.0_f64 / 9.0_f64 * t3281 * t13173 + 2.0_f64 / 3.0_f64 * t446 * t13177 + 4.0_f64 / 3.0_f64 * t446 * t13181;
    t13184
}
