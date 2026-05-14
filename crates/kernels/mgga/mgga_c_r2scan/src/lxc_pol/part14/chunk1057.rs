//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1057/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1057<F: Float>(t11199: F, t11475: F, t3262: F, t11020: F, t12203: F, t11325: F, t11486: F, t10622: F, t12098: F, t3275: F, t10940: F, t12033: F, t10634: F, t3465: F, t40383: F, t11336: F, t37327: F, t40297: F) -> (F, F, F, F, F, F, F, F) {
    let t41168 = 3.0 / 2.0 * t3262 * t11199 * t11475;
    let t41170 = 5.0 / 16.0 * t11020 * t12203;
    let t41173 = 15.0 / 8.0 * t3262 * t11325 * t11486;
    let t41176 = 5.0 / 16.0 * t3275 * t12098 * t10622;
    let t41179 = t10940 * t12033 / 4.0;
    let t41182 = 15.0 / 8.0 * t3262 * t12098 * t10634;
    let t41185 = 3.0 / 2.0 * t3262 * t3465 * t40383;
    let t41188 = 15.0 / 8.0 * t37327 * t11336 * t40297;
    (t41168, t41170, t41173, t41176, t41179, t41182, t41185, t41188)
}
