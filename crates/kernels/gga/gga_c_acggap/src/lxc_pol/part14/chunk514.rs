//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 514/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk514<F: Float>(t1165: F, t407: F, t991: F, t1163: F, t1171: F, t3370: F, t1170: F, t1162: F, t3088: F, t1037: F, t3073: F, t322: F, t944: F, t1172: F, t1530: F, t301: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3427 = t1165 * t991 * t407;
    let t3428 = t1163 * t3427;
    let t3430 = t3370 * t1171;
    let t3431 = t1170 * t3430;
    let t3451 = t3088 * t1162;
    let t3453 = t1165 * t1037 * t407;
    let t3454 = t3451 * t3453;
    let t3456 = t3073 * t1162;
    let t3457 = t944 * t322;
    let t3462 = t1530 * t1172;
    let t3463 = t944 * t301;
    (t3427, t3428, t3431, t3451, t3453, t3454, t3456, t3457, t3462, t3463)
}
