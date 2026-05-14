//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 905/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk905<F: Float>(t3031: F, t956: F, t265: F, t9825: F, t9630: F, t1211: F, t3542: F, t1207: F, t3574: F, t3573: F, t401: F, t396: F, t9725: F, t9728: F, t3549: F, t3005: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10874 = t956 * t3031;
    let t10877 = t265 * t9825;
    let t10884 = t265 * t9630;
    let t10888 = t3542 * t1211;
    let t10893 = t1207 * t3574;
    let t10897 = 1.0 / t3573 / t401;
    let t10898 = t396 * t10897;
    let t10923 = 0.16068111111111111111e1 * t9725;
    let t10924 = 0.46308888888888888888e0 * t9728;
    let t10936 = t1207 * t3549;
    let t10945 = 0.53272592592592592592e-1 * t9725;
    let t10960 = t956 * t3005;
    (t10874, t10877, t10884, t10888, t10893, t10898, t10923, t10924, t10936, t10945, t10960)
}
