//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 907/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk907<F: Float>(t1411: F, t25330: F, t3494: F, t8077: F, t1415: F, t1286: F, t7706: F, t13293: F, t3484: F, t3482: F, t1056: F) -> (F, F, F, F, F, F) {
    let t25331 = t1411 * t25330;
    let t25333 = t3494 * t8077;
    let t25334 = t1415 * t25333;
    let t25335 = t1411 * t25334;
    let t25337 = t7706 * t1286;
    let t25338 = t13293 * t25337;
    let t25339 = t3484 * t25338;
    let t25340 = t3482 * t25339;
    let t25342 = t7706 * t1056;
    (t25331, t25335, t25337, t25338, t25340, t25342)
}
