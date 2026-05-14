//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 773/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk773<F: Float>(t262: F, t39044: F, t34938: F, t7501: F, t8672: F, t321: F, t8704: F, t8640: F, t333: F, t7198: F, t352: F, t7204: F, t3807: F, t8639: F, t8642: F, t1462: F, t236: F, t498: F, t7231: F, t8517: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t39045 = t262 * t39044;
    let t39046 = t34938 * t39045;
    let t39048 = t7501 * t8672;
    let t39055 = t8704 * t321;
    let t39056 = t262 * t39055;
    let t39057 = t8640 * t39056;
    let t39059 = t8704 * t333;
    let t39060 = t262 * t39059;
    let t39061 = t7198 * t39060;
    let t39063 = t8704 * t352;
    let t39064 = t262 * t39063;
    let t39065 = t7204 * t39064;
    let t39068 = t3807 * t8639 * t8642;
    let t39073 = t8517 * t7231 * t236 * t1462 * t498;
    (t39045, t39046, t39048, t39055, t39056, t39057, t39059, t39060, t39061, t39063, t39064, t39065, t39068, t39073)
}
