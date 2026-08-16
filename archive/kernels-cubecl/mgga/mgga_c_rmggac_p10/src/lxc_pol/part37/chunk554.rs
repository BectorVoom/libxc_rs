//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 554/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk554<F: Float>(t13926: F, t13938: F, t13943: F, t2123: F, t2211: F, t118: F, t2085: F, t3191: F, t2228: F, t326: F, t650: F, t699: F, t838: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14487 = F::cast_from(0.88507694033737208928e-3_f64) * t13926;
    let t14491 = F::cast_from(0.14464861606874801909e-3_f64) * t13938;
    let t14493 = F::cast_from(0.12857654761666490586e-3_f64) * t13943;
    let t14498 = t2211 * t2123;
    let t14500 = F::cast_from(0.39914139006212695214e-1_f64) * t118 * t14498;
    let t14504 = t3191 * t2085;
    let t14505 = F::cast_from(0.90915538847484472429e-2_f64) * t14504;
    let t14506 = t326 * t2228;
    let t14507 = t14506 * t650;
    let t14508 = F::cast_from(0.34093327067806677161e-2_f64) * t14507;
    let t14509 = t838 * t699;
    (t14487, t14491, t14493, t14498, t14500, t14505, t14506, t14508, t14509)
}
