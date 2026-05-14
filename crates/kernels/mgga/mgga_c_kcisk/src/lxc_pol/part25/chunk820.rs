//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 820/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk820<F: Float>(t1871: F, t5174: F, t4811: F, t4818: F, t1755: F, t695: F, t1907: F, t5211: F, t1904: F, t5217: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t11658 = t5174 * t1871;
    let t11659 = t11658 * sigma2;
    let t11663 = t4811 * t4818;
    let t11676 = t1755 * t695;
    let t11691 = t5211 * t1907;
    let t11694 = t1904 * t5217;
    (t11658, t11659, t11663, t11676, t11691, t11694)
}
