//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 655/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk655(t2: f64, t9132: f64, t143: f64, t7760: f64, t2118: f64, t458: f64, t153: f64, t525: f64, t631: f64, t637: f64, t7242: f64, t576: f64, t8232: f64) -> (f64, f64, f64, f64, f64) {
    let t9217 = t9132 * t2;
    let t9224 = t7760 * t143;
    let t9241 = t458 * t2118;
    let t9252 = 1.0_f64 / t153 / t631 / t637 / t525 / t7242 / 4.0_f64;
    let t9270 = t8232 * t576;
    (t9217, t9224, t9241, t9252, t9270)
}
