//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1353/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1353(t26287: f64, t26298: f64, t26908: f64, t26913: f64, t26916: f64, t26929: f64, t26938: f64, t26941: f64, t26944: f64, t26947: f64, t3103: f64, t3109: f64, t3116: f64, t3132: f64, t3133: f64, t3235: f64, t4386: f64, t4387: f64, t8451: f64, t8460: f64, t8475: f64) -> f64 {
    let t26949 = 0.18933502127510156893e0_f64 * t26908 - 0.12209704640613106892e2_f64 * t26913 - 0.13735917720689745254e2_f64 * t3132 * t26916 * t3133 + 0.27471835441379490507e2_f64 * t3103 * t26916 * t3109 - 0.10866451862235947318e0_f64 * t4386 * t4387 * t26287 + 0.65198711173415683908e-1_f64 * t4386 * t3235 * t26298 - 0.94667510637550784466e0_f64 * t3116 * t8460 * t26929 - 0.2840025319126523534e0_f64 * t3116 * t8451 * t8475 - 0.36629113921839320676e2_f64 * t26938 - 0.24419409281226213784e2_f64 * t26941 + 0.6104852320306553446e1_f64 * t26944 - 0.28977204965962526182e-1_f64 * t26947;
    t26949
}
