//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1010/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1010<F: Float>(t17874: F, t17875: F, t16617: F, t7303: F, t7302: F, t16980: F, t719: F, t735: F, t1935: F, t17111: F, t5290: F, t5289: F, t1930: F, t718: F, t7304: F, t17855: F, t5322: F) -> (F, F, F, F, F, F, F, F) {
    let t17876 = t17874 * t17875;
    let t17878 = t7303 * t16617;
    let t17879 = t7302 * t17878;
    let t17881 = t719 * t16980;
    let t17882 = t735 * t17881;
    let t17883 = t1935 * t17882;
    let t17885 = t5290 * t17111;
    let t17886 = t5289 * t17885;
    let t17888 = t1930 * t718;
    let t17889 = t17888 * t7304;
    let t17891 = t5322 * t17855;
    (t17876, t17878, t17879, t17883, t17885, t17886, t17889, t17891)
}
