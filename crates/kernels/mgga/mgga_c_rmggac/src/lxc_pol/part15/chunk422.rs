//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 422/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk422<F: Float>(t4153: F, t431: F, t1037: F, t409: F, t1040: F, t179: F, t4052: F, t1045: F, t973: F, t1042: F, t1003: F, t230: F) -> (F, F, F, F, F, F, F) {
    let t4155 = F::new(0.5848223622634646207e0) * t431 * t4153;
    let t4157 = F::new(1.0) / t1037 / t409;
    let t4160 = F::new(1.0) / t1040 / t179;
    let t4161 = t4157 * t4052 * t4160;
    let t4163 = F::new(0.10254018858216406658e4) * t431 * t4161;
    let t4167 = t1045 * t973;
    let t4169 = t1045 * t1042;
    let t4179 = F::new(1.0) / t1003 / t230;
    (t4155, t4157, t4160, t4163, t4167, t4169, t4179)
}
