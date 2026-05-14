//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 672/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk672<F: Float>(t1310: F, t13893: F, t12829: F, t403: F, t1311: F, t164: F, t25: F, t3951: F, t10471: F, t1337: F, t140: F, t3480: F, t3737: F, t213: F, t300: F, t425: F) -> (F, F, F, F, F, F, F, F) {
    let t13894 = t1310 * t13893;
    let t13895 = t403 * t12829;
    let t13900 = t164 * t1311;
    let t13917 = t25 * t3951;
    let t13955 = t140 * t10471 * t1337;
    let t13959 = t140 * t3737 * t3480;
    let t13964 = t213 * t300;
    let t13966 = 0.14055920378328537299e-1 * t13964 * t425;
    (t13894, t13895, t13900, t13917, t13955, t13959, t13964, t13966)
}
