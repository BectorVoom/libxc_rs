//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1145/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1145<F: Float>(t11036: F, t9657: F, t1070: F, t31764: F, t2928: F, t37028: F, t11033: F, t2938: F, t3366: F, t9640: F, t37039: F, t37041: F, t37066: F, t37076: F, t40822: F, t40841: F, t40845: F, t41872: F, t42524: F, t42526: F, t42528: F, t42530: F, t42532: F) -> F {
    let t42534 = t11036 * t9657;
    let t42536 = t31764 * t1070;
    let t42539 = t37028 * t2928;
    let t42541 = t11033 * t2938;
    let t42543 = t9640 * t3366;
    let t42546 = -F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t42524 + t42526 / F::cast_from(4.0_f64) + t42528 / F::cast_from(8.0_f64) + t41872 + t40822 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t42530 - t42532 / F::cast_from(2.0_f64) - t42534 / F::cast_from(4.0_f64) - t42536 / F::cast_from(8.0_f64) + F::cast_from(11.0_f64) / F::cast_from(9.0_f64) * t37041 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t42539 + t37039 - t40841 + t40845 - t42541 / F::cast_from(3.0_f64) + t42543 / F::cast_from(3.0_f64) - F::cast_from(11.0_f64) / F::cast_from(9.0_f64) * t37066 + t37076;
    t42546
}
