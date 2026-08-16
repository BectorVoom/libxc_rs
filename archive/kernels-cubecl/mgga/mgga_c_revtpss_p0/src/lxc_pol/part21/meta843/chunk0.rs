//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3156/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3156<F: Float>(t16903: F, t698: F, t141: F, t3417: F, t56192: F, t56196: F, t56201: F, t56205: F, t43865: F, t43883: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F) -> (F, F, F, F, F, F) {
    let t58186 = t698 * t16903;
    let t58189 = t141 * t3417 * t56192;
    let t58192 = t141 * t3417 * t56196;
    let t58195 = t141 * t3417 * t56201;
    let t58198 = t141 * t3417 * t56205;
    let t58200 = -F::cast_from(0.45908888888888888888e0_f64) * t43865 + F::cast_from(0.68863333333333333332e0_f64) * t43883 - F::cast_from(0.16068111111111111111e1_f64) * t43888 + F::cast_from(0.68863333333333333332e0_f64) * t43890 + F::cast_from(0.13772666666666666666e1_f64) * t43892 - F::cast_from(0.103295e1_f64) * t43894 - F::cast_from(0.17215833333333333333e0_f64) * t43896 - F::cast_from(0.83356000000000000001e0_f64) * t58186 - F::cast_from(0.104195e0_f64) * t58189 - F::cast_from(0.104195e0_f64) * t58192 - F::cast_from(0.62517000000000000001e0_f64) * t58195 - F::cast_from(0.34731666666666666667e-1_f64) * t58198;
    (t58186, t58189, t58192, t58195, t58198, t58200)
}
