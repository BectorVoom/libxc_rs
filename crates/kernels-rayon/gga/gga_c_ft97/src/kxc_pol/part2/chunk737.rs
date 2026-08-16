//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 737/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk737(t3189: f64, t8506: f64, t1780: f64, t480: f64, t3195: f64, t11549: f64, t11550: f64, t11553: f64, t11558: f64, t11562: f64, t11567: f64, t11570: f64, t11574: f64, t11578: f64, t1901: f64, t3281: f64, t446: f64, t8227: f64, t8229: f64, t8233: f64, t8235: f64) -> f64 {
    let t11584 = t8506 * t3189;
    let t11587 = t1780 * t480;
    let t11588 = t11587 * t3195;
    let t11591 = t11549 - 4.0_f64 / 27.0_f64 * t11550 + 4.0_f64 / 27.0_f64 * t1901 * t11553 + 4.0_f64 / 27.0_f64 * t1901 * t11558 + 2.0_f64 / 3.0_f64 * t446 * t11562 - t11567 + t446 * t11570 / 3.0_f64 - 4.0_f64 / 9.0_f64 * t3281 * t11574 + 4.0_f64 / 27.0_f64 * t11578 - 2.0_f64 / 27.0_f64 * t8227 - 2.0_f64 / 9.0_f64 * t8229 - 8.0_f64 / 81.0_f64 * t8233 + 2.0_f64 / 81.0_f64 * t8235 + 4.0_f64 / 9.0_f64 * t1901 * t11584 - 4.0_f64 / 27.0_f64 * t1901 * t11588;
    t11591
}
