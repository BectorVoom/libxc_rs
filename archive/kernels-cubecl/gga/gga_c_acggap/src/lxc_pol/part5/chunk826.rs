//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 826/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk826<F: Float>(t594: F, t8: F, t130: F, t1024: F, t56: F, t38: F, t22: F, t413: F, t406: F, t524: F) -> (F, F, F, F, F, F, F) {
    let t7321 = F::cast_from(1.0_f64) / t8 / t594;
    let t7322 = t130 * t7321;
    let t7335 = t56 * t1024;
    let t7508 = t38 * t38;
    let t7510 = F::cast_from(1.0_f64) / t22 / t7508;
    let t7599 = t130 * t413;
    let t7777 = F::cast_from(1.0_f64) / t7508;
    let t8401 = t524 * t406;
    (t7321, t7322, t7335, t7510, t7599, t7777, t8401)
}
