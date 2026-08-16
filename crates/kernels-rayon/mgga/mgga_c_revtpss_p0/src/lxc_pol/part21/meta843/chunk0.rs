//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3156/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3156(t16903: f64, t698: f64, t141: f64, t3417: f64, t56192: f64, t56196: f64, t56201: f64, t56205: f64, t43865: f64, t43883: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t58186 = t698 * t16903;
    let t58189 = t141 * t3417 * t56192;
    let t58192 = t141 * t3417 * t56196;
    let t58195 = t141 * t3417 * t56201;
    let t58198 = t141 * t3417 * t56205;
    let t58200 = -0.45908888888888888888e0_f64 * t43865 + 0.68863333333333333332e0_f64 * t43883 - 0.16068111111111111111e1_f64 * t43888 + 0.68863333333333333332e0_f64 * t43890 + 0.13772666666666666666e1_f64 * t43892 - 0.103295e1_f64 * t43894 - 0.17215833333333333333e0_f64 * t43896 - 0.83356000000000000001e0_f64 * t58186 - 0.104195e0_f64 * t58189 - 0.104195e0_f64 * t58192 - 0.62517000000000000001e0_f64 * t58195 - 0.34731666666666666667e-1_f64 * t58198;
    (t58186, t58189, t58192, t58195, t58198, t58200)
}
