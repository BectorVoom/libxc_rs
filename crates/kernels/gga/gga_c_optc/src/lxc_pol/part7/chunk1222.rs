//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1222/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1222<F: Float>(t26940: F, t8970: F, t26911: F, t3132: F, t3133: F, t4386: F, t8493: F, t9189: F, t26287: F, t26298: F, t26908: F, t26913: F, t26916: F, t26929: F, t26938: F, t3103: F, t3109: F, t3116: F, t3235: F, t4387: F, t8451: F, t8460: F, t8475: F) -> (F,) {
    let t26941 = t26940 * t8970;
    let t26944 = t3132 * t26911 * t3133;
    let t26947 = t4386 * t9189 * t8493;
    let t26949 = 0.18933502127510156893e0 * t26908 - 0.12209704640613106892e2 * t26913 - 0.13735917720689745254e2 * t3132 * t26916 * t3133 + 0.27471835441379490507e2 * t3103 * t26916 * t3109 - 0.10866451862235947318e0 * t4386 * t4387 * t26287 + 0.65198711173415683908e-1 * t4386 * t3235 * t26298 - 0.94667510637550784466e0 * t3116 * t8460 * t26929 - 0.2840025319126523534e0 * t3116 * t8451 * t8475 - 0.36629113921839320676e2 * t26938 - 0.24419409281226213784e2 * t26941 + 0.6104852320306553446e1 * t26944 - 0.28977204965962526182e-1 * t26947;
    (t26949,)
}
