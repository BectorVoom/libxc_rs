//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 916/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk916<F: Float>(t3308: F, t9296: F, t1577: F, t2651: F, t3597: F, t9292: F, t574: F, t9445: F, t2124: F, t9422: F, t3295: F, t9376: F, t3332: F, t2147: F, t10819: F, t11768: F, t12158: F, t12162: F, t12163: F, t12164: F, t12166: F, t12167: F) -> (F, F, F, F, F, F, F) {
    let t12511 = t3308 * t9296;
    let t12512 = t1577 * t12511;
    let t12515 = t2651 * t3597;
    let t12517 = t3308 * t9292;
    let t12518 = t574 * t12517;
    let t12520 = t3308 * t9445;
    let t12521 = t574 * t12520;
    let t12523 = t2124 * t9422;
    let t12524 = t3295 * t12523;
    let t12526 = t2124 * t9376;
    let t12527 = t3295 * t12526;
    let t12529 = t3332 * t9445;
    let t12530 = t2147 * t12529;
    let t12532 = 0.86682217400542685632e-1 * t12512 - t12158 + t12162 + t12163 - t12164 - 0.97574405393827830186e-2 * t11768 - t12166 + t12167 - 0.86682217400542685632e-1 * t12515 - 0.43341108700271342816e-1 * t12518 - 0.43341108700271342816e-1 * t12521 - 0.27439371595564631661e-1 * t12524 - 0.27439371595564631661e-1 * t12527 - t10819 + 0.21831846657716620896e-2 * t12530;
    (t12511, t12517, t12520, t12523, t12526, t12529, t12532)
}
