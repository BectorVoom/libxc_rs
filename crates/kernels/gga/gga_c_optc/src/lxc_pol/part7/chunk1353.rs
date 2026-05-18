//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1353/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1353<F: Float>(t26287: F, t26298: F, t26908: F, t26913: F, t26916: F, t26929: F, t26938: F, t26941: F, t26944: F, t26947: F, t3103: F, t3109: F, t3116: F, t3132: F, t3133: F, t3235: F, t4386: F, t4387: F, t8451: F, t8460: F, t8475: F) -> F {
    let t26949 = F::new(0.18933502127510156893e0) * t26908 - F::new(0.12209704640613106892e2) * t26913 - F::new(0.13735917720689745254e2) * t3132 * t26916 * t3133 + F::new(0.27471835441379490507e2) * t3103 * t26916 * t3109 - F::new(0.10866451862235947318e0) * t4386 * t4387 * t26287 + F::new(0.65198711173415683908e-1) * t4386 * t3235 * t26298 - F::new(0.94667510637550784466e0) * t3116 * t8460 * t26929 - F::new(0.2840025319126523534e0) * t3116 * t8451 * t8475 - F::new(0.36629113921839320676e2) * t26938 - F::new(0.24419409281226213784e2) * t26941 + F::new(0.6104852320306553446e1) * t26944 - F::new(0.28977204965962526182e-1) * t26947;
    t26949
}
