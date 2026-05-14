//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 903/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk903<F: Float>(t15821: F, t15822: F, t167: F, t19055: F, t6235: F, t1286: F, t8072: F, t1450: F, t3785: F, t1411: F, t2231: F, t5967: F, t1341: F, t3748: F, t8090: F, t8259: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22591 = t15821 + t15822;
    let t23050 = 2.0 * t167;
    let t25294 = t19055 * t6235;
    let t25296 = t8072 * t1286;
    let t25297 = t1450 * t25296;
    let t25298 = t3785 * t25297;
    let t25299 = t1411 * t25298;
    let t25301 = t2231 * t5967;
    let t25302 = t1341 * t25301;
    let t25303 = t3785 * t25302;
    let t25304 = t1411 * t25303;
    let t25306 = t3748 * t8090;
    let t25308 = t8259 * sigma0;
    (t22591, t23050, t25294, t25296, t25299, t25301, t25304, t25306, t25308)
}
