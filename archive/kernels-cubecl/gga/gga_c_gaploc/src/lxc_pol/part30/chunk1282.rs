//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1282/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1282<F: Float>(t10667: F, t2089: F, t28366: F, t28378: F, t1029: F, t7383: F, t9796: F, t8638: F, t9972: F, t11044: F, t2197: F, t11047: F) -> (F, F, F, F, F, F, F) {
    let t33118 = t2089 * t10667;
    let t33126 = F::cast_from(0.95857314884801874192e-1_f64) * t28366;
    let t33127 = F::cast_from(0.63904876589867916128e-1_f64) * t28378;
    let t33129 = t9796 * t1029 * t7383;
    let t33130 = F::cast_from(0.38342925953920749676e0_f64) * t33129;
    let t33132 = F::cast_from(0.21450293971110256002e1_f64) * t8638 * t9972;
    let t33134 = F::cast_from(0.23005755572352449806e2_f64) * t2197 * t11044;
    let t33136 = F::cast_from(0.23005755572352449806e2_f64) * t2197 * t11047;
    (t33118, t33126, t33127, t33130, t33132, t33134, t33136)
}
