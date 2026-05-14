//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 916/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk916<F: Float>(t1528: F, t236: F, t3351: F, t618: F, t9210: F, t7720: F, t9932: F, t39277: F, t8836: F, t8843: F, t2320: F, t39281: F, t1971: F, t6105: F, t8517: F, t10093: F, t498: F, t515: F) -> (F, F, F, F, F, F, F) {
    let t47263 = t3351 * t9210 * t236 * t618 * t1528;
    let t47265 = t7720 * t9932;
    let t47267 = t39277 * t8836;
    let t47269 = t39277 * t8843;
    let t47271 = t39281 * t2320;
    let t47275 = t8517 * t1971 * t236 * t6105;
    let t47280 = t3351 * t9210 * t515 * t10093 * t498;
    (t47263, t47265, t47267, t47269, t47271, t47275, t47280)
}
