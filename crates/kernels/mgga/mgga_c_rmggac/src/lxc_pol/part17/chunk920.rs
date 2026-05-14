//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 920/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk920<F: Float>(t2298: F, t26370: F, t17859: F, t9051: F, t9055: F, t9096: F, t9138: F, t2310: F, t38472: F, t9190: F, t9194: F, t10090: F, t16156: F, t7508: F, t9803: F, t2145: F, t27: F, t6463: F, t649: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t47340 = t26370 * t2298;
    let t47345 = t17859 * t9051;
    let t47347 = t17859 * t9055;
    let t47349 = t17859 * t9096;
    let t47351 = t17859 * t9138;
    let t47353 = t38472 * t2310;
    let t47355 = t17859 * t9190;
    let t47357 = t17859 * t9194;
    let t47359 = t16156 * t10090;
    let t47361 = t7508 * t9803;
    let t47365 = t2145 * t27 * t649 * t6463;
    (t47340, t47345, t47347, t47349, t47351, t47353, t47355, t47357, t47359, t47361, t47365)
}
