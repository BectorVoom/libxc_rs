//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 912/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk912<F: Float>(t3229: F, t797: F, t1103: F, t3128: F, t1053: F, t1102: F, t3162: F, t3446: F, t3453: F, t3165: F, t2201: F, t3602: F, t3613: F, t1060: F, t3216: F, t1058: F) -> (F, F, F, F, F, F, F, F) {
    let t12428 = t797 * t3229;
    let t12435 = t1103 * t3128;
    let t12437 = t1102 * t1053 * t12435;
    let t12440 = t3446 * t3453 * t3162;
    let t12443 = t3446 * t3453 * t3165;
    let t12446 = t2201 * t3613 * t3602;
    let t12448 = t1060 * t3216;
    let t12450 = t2201 * t1058 * t12448;
    (t12428, t12435, t12437, t12440, t12443, t12446, t12448, t12450)
}
