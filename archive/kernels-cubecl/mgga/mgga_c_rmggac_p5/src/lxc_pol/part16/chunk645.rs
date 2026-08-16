//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 645/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk645<F: Float>(t515: F, t9163: F, t1971: F, t1970: F, t209: F, t476: F, t618: F, t236: F, t7231: F, t498: F, t551: F, t7248: F) -> (F, F, F, F, F) {
    let t9164 = t515 * t9163;
    let t9165 = t1971 * t9164;
    let t9166 = t1970 * t9165;
    let t9169 = t618 * t476 * t209;
    let t9170 = t236 * t9169;
    let t9171 = t7231 * t9170;
    let t9172 = t1970 * t9171;
    let t9182 = t551 * t498;
    let t9183 = t236 * t9182;
    let t9184 = t7248 * t9183;
    (t9165, t9166, t9171, t9172, t9184)
}
