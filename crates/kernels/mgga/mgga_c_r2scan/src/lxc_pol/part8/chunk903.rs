//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 903/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk903<F: Float>(t1568: F, t8089: F, t7623: F, t2214: F, t2698: F, t514: F, t1616: F, t938: F, t2201: F, t785: F, t910: F, t2207: F, t2837: F, t783: F, t2842: F, t5100: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8243 = t1568 * t8089;
    let t8245 = 0.10975748638225852664e-1 * t7623 * t8243;
    let t8263 = t2214 * t2698;
    let t8265 = 0.19514881078765566037e-1 * t514 * t8263;
    let t8266 = t1616 * t938;
    let t8268 = t2201 * t785 * t8266;
    let t8270 = t1616 * t910;
    let t8272 = t2207 * t785 * t8270;
    let t8275 = t783 * t2837 * t1616;
    let t8277 = t5100 * t2842;
    (t8243, t8245, t8263, t8265, t8266, t8268, t8270, t8272, t8275, t8277)
}
