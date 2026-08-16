//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1130/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1130<F: Float>(t3591: F, t37972: F, t10872: F, t11736: F, t1615: F, t3320: F, t783: F, t978: F, t261: F, t3299: F, t7291: F, t3594: F, t37736: F) -> (F, F, F, F, F) {
    let t39552 = t37972 * t3591;
    let t39554 = t10872 * t11736;
    let t39558 = t783 * t978 * t1615 * t3320;
    let t39561 = t3299 * t261 * t7291;
    let t39563 = t37736 * t3594;
    (t39552, t39554, t39558, t39561, t39563)
}
