//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2802/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2802<F: Float>(t16791: F, t9546: F, t2586: F, t41146: F, t59162: F, t59135: F, t9523: F, t13005: F, t210: F, t214: F, t41209: F, t41212: F, t41217: F, t46838: F, t46844: F, t46855: F, t58139: F, t59198: F, t59204: F, t59206: F, t59214: F, t59216: F, t787: F) -> F {
    let t59218 = t9546 * t16791;
    let t59221 = t2586 * t41146 * t59162;
    let t59224 = t2586 * t9523 * t59135;
    let t59227 = -F::cast_from(0.79999999999999999996e-1_f64) * t13005 * t46838 * t59198 + F::cast_from(0.93333333333333333328e-1_f64) * t46844 - F::cast_from(0.19999999999999999999e-1_f64) * t46855 + t41209 + t41212 + F::cast_from(0.38888888888888888887e-1_f64) * t59204 + F::cast_from(0.15833333333333333333e-1_f64) * t59206 - F::cast_from(0.16666666666666666666e-2_f64) * t787 * t210 * t214 * t58139 + F::cast_from(0.16666666666666666666e-2_f64) * t59214 + F::cast_from(0.77777777777777777774e-2_f64) * t59216 - F::cast_from(0.52777777777777777776e-2_f64) * t59218 - F::cast_from(0.49999999999999999998e-2_f64) * t59221 + F::cast_from(0.16666666666666666666e-2_f64) * t59224 + F::cast_from(0.55555555555555555555e-3_f64) * t41217;
    t59227
}
