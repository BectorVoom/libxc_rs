//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 843/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk843(t3483: f64, t379: f64, t13220: f64, t13187: f64, t13190: f64, t13192: f64, t13196: f64, t13198: f64, t13201: f64, t13205: f64, t13209: f64, t13213: f64, t13217: f64, t1901: f64, t3281: f64, t446: f64, t9449: f64, t9451: f64, t9453: f64, t9457: f64) -> f64 {
    let t13221 = t3483 * t379;
    let t13222 = t13220 * t13221;
    let t13225 = -2.0_f64 / 9.0_f64 * t9449 - 2.0_f64 / 9.0_f64 * t9451 - 4.0_f64 / 27.0_f64 * t13187 + t13190 + 2.0_f64 / 3.0_f64 * t446 * t13192 + t13196 - 2.0_f64 / 9.0_f64 * t3281 * t13198 - 4.0_f64 / 27.0_f64 * t13201 + t9453 / 27.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t13205 - 4.0_f64 / 9.0_f64 * t1901 * t13209 + 4.0_f64 / 27.0_f64 * t1901 * t13213 - 2.0_f64 / 9.0_f64 * t1901 * t13217 - 4.0_f64 / 9.0_f64 * t1901 * t13222 - t9457;
    t13225
}
