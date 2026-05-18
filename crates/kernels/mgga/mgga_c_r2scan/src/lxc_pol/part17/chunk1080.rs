//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1080/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1080<F: Float>(t10949: F, t10992: F, t2315: F, t3446: F, t23194: F, t263: F, t3438: F, t6874: F, t10968: F, t6262: F, t6855: F, t10930: F, t158: F, t2304: F, t2317: F, t3434: F) -> (F, F, F, F) {
    let t38211 = t3446 * t10992 * t10949 * t2315;
    let t38225 = t3446 * t263 * t23194 * t3438 * t6874;
    let t38228 = t6855 * t6262 * t10968;
    let t38233 = t3434 * t2304 * t2317 * t158 * t10930;
    (t38211, t38225, t38228, t38233)
}
