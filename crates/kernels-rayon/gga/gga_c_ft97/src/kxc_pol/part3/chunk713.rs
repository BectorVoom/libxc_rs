//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 713/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk713(t13526: f64, t3724: f64, t1092: f64, t1771: f64, t3740: f64, t458: f64, t3743: f64, t11176: f64, t3747: f64, t222: f64, t226: f64, t1113: f64, t236: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13527 = t3724 * t13526;
    let t13538 = t1771 * t1092;
    let t13540 = t458 * t3740;
    let t13541 = 4.0_f64 / 27.0_f64 * t13540;
    let t13542 = t458 * t3743;
    let t13543 = 4.0_f64 / 9.0_f64 * t13542;
    let t13544 = t11176 * t3747;
    let t13580 = t222 * t226;
    let t13581 = t236 * t1113;
    (t13527, t13538, t13540, t13541, t13542, t13543, t13544, t13580, t13581)
}
