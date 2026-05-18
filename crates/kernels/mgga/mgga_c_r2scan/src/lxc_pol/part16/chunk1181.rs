//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1181/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1181<F: Float>(t10781: F, t8839: F, t10894: F, t3072: F, t10760: F, t29283: F, t6535: F, t11793: F, t2201: F, t3613: F, t12448: F, t3336: F) -> (F, F, F, F, F) {
    let t43115 = t10781 * t8839;
    let t43117 = t10894 * t3072;
    let t43120 = t6535 * t10760 * t29283;
    let t43123 = t2201 * t3613 * t11793;
    let t43126 = t2201 * t3336 * t12448;
    (t43115, t43117, t43120, t43123, t43126)
}
