//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1025/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1025<F: Float>(t10710: F, t10768: F, t29126: F, t10781: F, t8839: F, t10894: F, t3072: F, t10760: F, t29283: F, t6535: F, t11793: F, t2201: F, t3613: F, t12448: F, t3336: F, t1058: F, t1060: F, t9365: F) -> (F, F, F, F, F, F, F) {
    let t43111 = t10768 * t10710 * t29126;
    let t43115 = t10781 * t8839;
    let t43117 = t10894 * t3072;
    let t43120 = t6535 * t10760 * t29283;
    let t43123 = t2201 * t3613 * t11793;
    let t43126 = t2201 * t3336 * t12448;
    let t43130 = t2201 * t1058 * t1060 * t9365;
    (t43111, t43115, t43117, t43120, t43123, t43126, t43130)
}
