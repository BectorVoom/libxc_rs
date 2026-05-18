//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1191/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1191<F: Float>(t1805: F, t5165: F, t15068: F, t5062: F, t10796: F, t6717: F, t3474: F, t6697: F, t19630: F, t3338: F, t3337: F, t5096: F, t5172: F) -> (F, F, F, F, F, F) {
    let t19918 = t5165 * t1805;
    let t19920 = t15068 * t5062;
    let t19922 = t10796 * t6717;
    let t19924 = t3474 * t6697;
    let t19926 = t3338 * t19630;
    let t19927 = t3337 * t19926;
    let t19929 = t5172 * t5096;
    (t19918, t19920, t19922, t19924, t19927, t19929)
}
