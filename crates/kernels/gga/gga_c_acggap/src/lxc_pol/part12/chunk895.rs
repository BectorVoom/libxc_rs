//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 895/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk895<F: Float>(t2297: F, t4210: F, t13364: F, t31115: F, t1: F, t1170: F, t2065: F, t8461: F, t3196: F, t1530: F, t31114: F, t137: F, t524: F) -> (F, F, F, F, F, F, F) {
    let t33938 = t2297 * t4210;
    let t33940 = t31115 * t13364 * t33938;
    let t33944 = t1170 * t2065 * t8461 * t1;
    let t33945 = t2297 * t3196;
    let t33947 = t33944 * t13364 * t33945;
    let t33952 = t1530 * t31114;
    let t33953 = t137 * t524;
    (t33938, t33940, t33944, t33945, t33947, t33952, t33953)
}
