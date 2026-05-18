//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1234/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1234<F: Float>(t481: F, t8601: F, t3262: F, t3263: F, t12428: F, t792: F, t10997: F, t3275: F, t105: F, t3055: F, t97: F, t10669: F) -> (F, F, F) {
    let t43717 = t8601 * t481;
    let t43720 = F::new(3.0) / F::new(4.0) * t3262 * t3263 * t43717;
    let t43721 = t12428 * t792;
    let t43724 = F::new(45.0) / F::new(64.0) * t3275 * t10997 * t43721;
    let t43726 = t97 * t105 * t3055;
    let t43728 = F::new(3.0) / F::new(4.0) * t43726 * t10669;
    (t43720, t43724, t43728)
}
