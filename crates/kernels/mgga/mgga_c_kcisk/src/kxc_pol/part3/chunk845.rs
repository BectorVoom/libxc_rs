//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 845/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk845<F: Float>(t13950: F, t1415: F, t1411: F, t10471: F, t1337: F, t140: F, t1343: F, t3480: F, t3737: F, t3488: F, t1333: F, t3909: F, t213: F, t300: F, t425: F, t1350: F, t1387: F) -> (F, F, F, F, F, F, F, F) {
    let t13951 = t1415 * t13950;
    let t13952 = t1411 * t13951;
    let t13955 = t140 * t10471 * t1337;
    let t13956 = t13955 * t1343;
    let t13959 = t140 * t3737 * t3480;
    let t13960 = t13959 * t3488;
    let t13962 = t1333 * t3909;
    let t13964 = t213 * t300;
    let t13966 = 0.14055920378328537299e-1 * t13964 * t425;
    let t13967 = t1387 * t1350;
    (t13952, t13956, t13959, t13960, t13962, t13964, t13966, t13967)
}
