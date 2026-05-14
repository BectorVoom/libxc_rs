//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1029/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1029<F: Float>(t16643: F, t17163: F, t7618: F, t2013: F, t5470: F, t7218: F, t5480: F, t7591: F, t5471: F, t7602: F, t7574: F, t1772: F, t7581: F, t16562: F, t41: F, t1689: F, t260: F, t67: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18244 = 0.15476481481481481481e-2 * t16643;
    let t18253 = t17163 * t7618;
    let t18254 = t2013 * t18253;
    let t18260 = t5470 * t7218;
    let t18264 = 0.15991812192157615301e-1 * t7591 * t5480;
    let t18270 = 0.59969295720591057378e-2 * t5471 * t7602;
    let t18271 = t7574 * sigma2;
    let t18272 = t18271 * t1772;
    let t18275 = t7581 * t5480;
    let t18289 = t16562 * t41;
    let t18306 = t260 * t67 * t1689;
    (t18244, t18254, t18260, t18264, t18270, t18272, t18275, t18289, t18306)
}
