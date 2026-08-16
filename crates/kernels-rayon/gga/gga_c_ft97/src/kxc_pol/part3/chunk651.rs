//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 651/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk651(t454: f64, t8232: f64, t463: f64, t480: f64, t1637: f64, t482: f64, t89: f64, t100: f64, t8326: f64, t104: f64, t7943: f64, t1786: f64, t488: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8485 = t8232 * t454;
    let t8506 = t463 * t480;
    let t8516 = t89 * t1637 * t482;
    let t8518 = t8326 * t100;
    let t8534 = 28.0_f64 / 81.0_f64 * t89 * t7943 * t104;
    let t8557 = t1786 * t488;
    (t8485, t8506, t8516, t8518, t8534, t8557)
}
