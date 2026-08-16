//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2802/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2802(t16791: f64, t9546: f64, t2586: f64, t41146: f64, t59162: f64, t59135: f64, t9523: f64, t13005: f64, t210: f64, t214: f64, t41209: f64, t41212: f64, t41217: f64, t46838: f64, t46844: f64, t46855: f64, t58139: f64, t59198: f64, t59204: f64, t59206: f64, t59214: f64, t59216: f64, t787: f64) -> f64 {
    let t59218 = t9546 * t16791;
    let t59221 = t2586 * t41146 * t59162;
    let t59224 = t2586 * t9523 * t59135;
    let t59227 = -0.79999999999999999996e-1_f64 * t13005 * t46838 * t59198 + 0.93333333333333333328e-1_f64 * t46844 - 0.19999999999999999999e-1_f64 * t46855 + t41209 + t41212 + 0.38888888888888888887e-1_f64 * t59204 + 0.15833333333333333333e-1_f64 * t59206 - 0.16666666666666666666e-2_f64 * t787 * t210 * t214 * t58139 + 0.16666666666666666666e-2_f64 * t59214 + 0.77777777777777777774e-2_f64 * t59216 - 0.52777777777777777776e-2_f64 * t59218 - 0.49999999999999999998e-2_f64 * t59221 + 0.16666666666666666666e-2_f64 * t59224 + 0.55555555555555555555e-3_f64 * t41217;
    t59227
}
