//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 613/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk613<F: Float>(t384: F, t5251: F, t1426: F, t175: F, t4818: F, t1298: F, t360: F, t1089: F, t368: F, t1032: F, t1423: F, t513: F, t922: F, t1095: F, t1175: F, t1181: F, t1532: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5253 = 0.17149607247227894789e-2 * t384 * t5251;
    let t5255 = t1426 * t175 * t4818;
    let t5258 = t1298 * t360;
    let t5260 = t1089 * t368 * t5258;
    let t5263 = t1032 * t1423;
    let t5265 = t513 * t922;
    let t5267 = t1426 * t1095 * t5265;
    let t5270 = t1175 * t360;
    let t5272 = t1181 * t1532 * t5270;
    (t5253, t5255, t5258, t5260, t5263, t5265, t5267, t5270, t5272)
}
