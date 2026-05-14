//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1086/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1086<F: Float>(t481: F, t8601: F, t3262: F, t3263: F, t12428: F, t792: F, t10997: F, t3275: F, t105: F, t3055: F, t97: F, t10669: F, t12570: F, t3276: F, t3582: F, t40705: F) -> (F, F, F, F, F) {
    let t43717 = t8601 * t481;
    let t43720 = 3.0 / 4.0 * t3262 * t3263 * t43717;
    let t43721 = t12428 * t792;
    let t43724 = 45.0 / 64.0 * t3275 * t10997 * t43721;
    let t43726 = t97 * t105 * t3055;
    let t43728 = 3.0 / 4.0 * t43726 * t10669;
    let t43729 = t12570 * t792;
    let t43732 = 15.0 / 16.0 * t3262 * t3276 * t43729;
    let t43735 = 5.0 / 8.0 * t3275 * t40705 * t3582;
    (t43720, t43724, t43728, t43732, t43735)
}
