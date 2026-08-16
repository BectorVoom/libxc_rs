//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 920/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk920(t3972: f64, t713: f64, t729: f64, t762: f64, t10048: f64, t10062: f64, t10064: f64, t10090: f64, t14212: f64, t14215: f64, t14219: f64, t14223: f64, t14224: f64, t14228: f64, t14232: f64, t14233: f64, t14240: f64, t14242: f64, t3281: f64, t446: f64) -> f64 {
    let t14245 = t3972 * t713;
    let t14247 = t729 * t762 * t14245;
    let t14251 = t14212 + 2.0_f64 / 3.0_f64 * t446 * t14215 + 4.0_f64 / 9.0_f64 * t3281 * t14219 - t14223 - 4.0_f64 / 81.0_f64 * t14224 - 2.0_f64 / 3.0_f64 * t446 * t14228 + t14232 - 4.0_f64 / 27.0_f64 * t14233 + t10048 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t10062 - 2.0_f64 / 9.0_f64 * t10064 - t14240 + 2.0_f64 / 3.0_f64 * t446 * t14242 + 2.0_f64 / 3.0_f64 * t446 * t14247 - 2.0_f64 / 27.0_f64 * t10090;
    t14251
}
