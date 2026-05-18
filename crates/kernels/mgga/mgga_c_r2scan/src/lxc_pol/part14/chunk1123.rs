//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1123/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1123<F: Float>(t261: F, t3299: F, t7291: F, t3594: F, t37736: F, t10879: F, t11741: F, t10907: F, t2201: F, t3602: F, t11824: F, t2207: F, t3336: F) -> (F, F, F, F, F) {
    let t39561 = t3299 * t261 * t7291;
    let t39563 = t37736 * t3594;
    let t39565 = t10879 * t11741;
    let t39569 = t2201 * t10907 * t3602;
    let t39572 = t2207 * t3336 * t11824;
    (t39561, t39563, t39565, t39569, t39572)
}
