//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1111/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1111<F: Float>(t28378: F, t1029: F, t7383: F, t9796: F, t8638: F, t9972: F, t11044: F, t2197: F, t11047: F, t2028: F, t3038: F, t7275: F, t787: F, t10012: F, t10627: F, t15482: F, t22633: F) -> (F, F, F, F, F, F, F) {
    let t33127 = 0.63904876589867916128e-1 * t28378;
    let t33129 = t9796 * t1029 * t7383;
    let t33130 = 0.38342925953920749676e0 * t33129;
    let t33132 = 0.21450293971110256002e1 * t8638 * t9972;
    let t33134 = 0.23005755572352449806e2 * t2197 * t11044;
    let t33136 = 0.23005755572352449806e2 * t2197 * t11047;
    let t33145 = 0.79445533226334281486e-1 * t787 * t7275 * t3038 * t2028;
    let t33148 = t10012 * t10627;
    let t33151 = 0.5680433474654925878e0 * t22633 * t15482 * t33148;
    (t33127, t33130, t33132, t33134, t33136, t33145, t33151)
}
