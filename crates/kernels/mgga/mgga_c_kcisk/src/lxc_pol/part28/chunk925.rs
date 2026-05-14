//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 925/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk925<F: Float>(t17064: F, t740: F, t11226: F, t5320: F, t1871: F, t7399: F, t11774: F, t718: F, t6973: F, t7336: F, t11807: F, t79: F, t1934: F, t2532: F, t2585: F, t1872: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t17816 = t17064 * t740;
    let t17846 = t11226 * t5320;
    let t17861 = t7399 * t1871;
    let t17862 = t17861 * sigma2;
    let t17874 = t11774 * t718;
    let t17933 = t6973 * t5320;
    let t17936 = t7336 * t718;
    let t17939 = t79 * t11807;
    let t17969 = t1934 * t2532;
    let t17975 = t740 * t2585;
    let t17976 = t1872 * t17975;
    (t17816, t17846, t17861, t17862, t17874, t17933, t17936, t17939, t17969, t17975, t17976)
}
