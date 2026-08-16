//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 718/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk718(t11286: f64, t11312: f64, t409: f64, t64: f64, t1602: f64, t939: f64, t11084: f64, t7906: f64, t11089: f64, t1631: f64, t11146: f64, t11225: f64, t11232: f64, t11233: f64, t11241: f64, t11246: f64, t11247: f64, t11251: f64, t1604: f64, t1605: f64, t1625: f64, t1751: f64, t3076: f64, t3077: f64, t3101: f64, t372: f64, t399: f64, t428: f64, t6426: f64, t7877: f64, t7879: f64) -> f64 {
    let t11313 = t11286 + t11312;
    let t11315 = t64 * t409 * t11313;
    let t11318 = t1602 * t939;
    let t11321 = t7906 * t11084;
    let t11324 = t1631 * t11089;
    let t11327 = t1631 * t11146;
    let t11330 = 4.0_f64 * t3076 * t11225 * t428 + 2.0_f64 * t3076 * t3077 * t1751 - 0.46509801892875584e-2_f64 * t11232 * t11233 * t1625 + 0.46509801892875584e-1_f64 * t7877 * t6426 * t7879 + 0.93019603785751168e-2_f64 * t11241 * t11233 * t1604 + 0.77462893625097599763e-3_f64 * t11246 * t11247 * t1604 - 2.0_f64 * t11251 - t11315 - 0.11854761295685025975e-1_f64 * t3101 * t399 + 0.46509801892875584e-1_f64 * t11318 * t1605 - 0.11619434043764639964e-3_f64 * t372 * t11321 + 0.46509801892875584e-2_f64 * t372 * t11324 + 0.23254900946437792e-2_f64 * t372 * t11327;
    t11330
}
