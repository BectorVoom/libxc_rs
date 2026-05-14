//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1051/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1051<F: Float>(t10698: F, t12506: F, t12517: F, t1584: F, t29936: F, t3308: F, t574: F, t12520: F, t12463: F, t2207: F, t3336: F, t481: F, t8601: F, t12428: F, t792: F, t105: F, t3055: F, t97: F) -> (F, F, F, F, F, F, F, F) {
    let t43695 = t10698 * t12506;
    let t43697 = t1584 * t12517;
    let t43700 = t574 * t3308 * t29936;
    let t43702 = t1584 * t12520;
    let t43705 = t2207 * t3336 * t12463;
    let t43717 = t8601 * t481;
    let t43721 = t12428 * t792;
    let t43726 = t97 * t105 * t3055;
    (t43695, t43697, t43700, t43702, t43705, t43717, t43721, t43726)
}
