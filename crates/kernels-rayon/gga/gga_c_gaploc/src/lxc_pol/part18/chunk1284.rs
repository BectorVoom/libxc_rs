//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1284/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1284(t10667: f64, t2089: f64, t28366: f64, t28378: f64, t1029: f64, t7383: f64, t9796: f64, t8638: f64, t9972: f64, t11044: f64, t2197: f64, t11047: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33118 = t2089 * t10667;
    let t33126 = 0.95857314884801874192e-1_f64 * t28366;
    let t33127 = 0.63904876589867916128e-1_f64 * t28378;
    let t33129 = t9796 * t1029 * t7383;
    let t33130 = 0.38342925953920749676e0_f64 * t33129;
    let t33132 = 0.21450293971110256002e1_f64 * t8638 * t9972;
    let t33134 = 0.23005755572352449806e2_f64 * t2197 * t11044;
    let t33136 = 0.23005755572352449806e2_f64 * t2197 * t11047;
    (t33118, t33126, t33127, t33130, t33132, t33134, t33136)
}
