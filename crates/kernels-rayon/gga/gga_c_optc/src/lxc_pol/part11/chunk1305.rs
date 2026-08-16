//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1305/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1305(t10416: f64, t16655: f64, t16933: f64, t3788: f64, t13842: f64, t41392: f64, t845: f64, t16929: f64, t24021: f64, t56677: f64, t7504: f64, t2416: f64, t4815: f64, t4818: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57246 = 24.0_f64 * t10416 * t16655;
    let t57248 = 0.14035736153892489771e2_f64 * t3788 * t16933;
    let t57251 = 0.61523382126046769581e4_f64 * t845 * t13842 * t41392;
    let t57253 = 0.4155781415850207192e3_f64 * t3788 * t16929;
    let t57257 = 0.12304676425209353917e5_f64 * t845 * t24021 * t56677 * t7504;
    let t57260 = 36.0_f64 * t2416 * t4815 * t4818;
    (t57246, t57248, t57251, t57253, t57257, t57260)
}
