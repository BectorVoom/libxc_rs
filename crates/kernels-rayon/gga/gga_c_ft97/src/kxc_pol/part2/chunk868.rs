//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 868/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk868(t807: f64, t9542: f64, t1092: f64, t1771: f64, t3740: f64, t458: f64, t3743: f64, t11176: f64, t3747: f64, t13315: f64, t9568: f64, t92: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13531 = t807 * t9542;
    let t13538 = t1771 * t1092;
    let t13540 = t458 * t3740;
    let t13541 = 4.0_f64 / 27.0_f64 * t13540;
    let t13542 = t458 * t3743;
    let t13543 = 4.0_f64 / 9.0_f64 * t13542;
    let t13544 = t11176 * t3747;
    let t13546 = t9568 * t13315;
    let t13547 = t92 * t13546;
    (t13531, t13538, t13540, t13541, t13542, t13543, t13544, t13547)
}
