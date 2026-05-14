//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 946/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk946<F: Float>(t3521: F, t7025: F, t6771: F, t695: F, t1060: F, t4604: F, t3293: F, t7023: F, t2372: F, t4597: F, t11279: F, t3290: F, t2364: F, t4609: F, t4684: F, t1824: F, t6714: F) -> (F, F, F, F, F, F) {
    let t16729 = 0.13140859333333333333e-2 * t3521 * t7025;
    let t16730 = t6771 * t695;
    let t16731 = t16730 * t1060;
    let t16732 = t4604 * t16731;
    let t16736 = t4604 * t7023 * t3293;
    let t16739 = t2372 * t4597;
    let t16741 = t11279 * t16739 * t3290;
    let t16745 = t4609 * t2364 * t4684;
    let t16749 = t4609 * t6714 * t1824;
    (t16729, t16732, t16736, t16741, t16745, t16749)
}
