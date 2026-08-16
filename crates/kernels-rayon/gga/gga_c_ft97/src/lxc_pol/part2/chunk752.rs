//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 752/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk752(t11811: f64, t1876: f64, t11810: f64, t1820: f64, t3238: f64, t83: f64, t1882: f64, t3231: f64, t11593: f64, t11596: f64, t11601: f64, t11606: f64, t11610: f64, t11612: f64, t11615: f64, t11620: f64, t11625: f64, t11628: f64, t11632: f64, t11803: f64, t11807: f64, t1901: f64, t28: f64, t446: f64, t89: f64) -> (f64, f64) {
    let t11812 = t11811 * t1876;
    let t11813 = t11810 * t11812;
    let t11816 = t3238 * t1820;
    let t11817 = t83 * t11816;
    let t11821 = 2.0_f64 / 9.0_f64 * t1882 * t3231;
    let t11822 = 4.0_f64 / 9.0_f64 * t11593 * t11596 + 4.0_f64 / 9.0_f64 * t11593 * t11601 + 8.0_f64 / 9.0_f64 * t11593 * t11606 + t11610 - t11612 - 2.0_f64 / 3.0_f64 * t446 * t11615 - 2.0_f64 * t446 * t11620 - 2.0_f64 / 3.0_f64 * t446 * t11625 - 2.0_f64 * t446 * t11628 - t11632 + t89 * t28 * t11803 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t11807 - 4.0_f64 / 3.0_f64 * t1901 * t11813 - t446 * t11817 / 3.0_f64 + t11821;
    (t11816, t11822)
}
