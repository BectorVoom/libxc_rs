//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 720/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk720<F: Float>(t1008: F, t1576: F, t1298: F, t322: F, t1089: F, t175: F, t384: F, t1426: F, t4818: F, t360: F, t368: F, t1032: F, t1423: F) -> (F, F, F, F, F, F, F, F) {
    let t5243 = F::cast_from(0.85748036236139473944e-3_f64) * t1008 * t1576;
    let t5249 = t1298 * t322;
    let t5251 = t1089 * t175 * t5249;
    let t5253 = F::cast_from(0.17149607247227894789e-2_f64) * t384 * t5251;
    let t5255 = t1426 * t175 * t4818;
    let t5258 = t1298 * t360;
    let t5260 = t1089 * t368 * t5258;
    let t5263 = t1032 * t1423;
    (t5243, t5249, t5251, t5253, t5255, t5258, t5260, t5263)
}
