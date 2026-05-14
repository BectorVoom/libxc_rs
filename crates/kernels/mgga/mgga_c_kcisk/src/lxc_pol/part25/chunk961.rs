//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 961/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk961<F: Float>(t1636: F, t6684: F, t17031: F, t5182: F, t6689: F, t10426: F, t2364: F, t5068: F, t1801: F, t5183: F, t5186: F, t6763: F, t6758: F, t6674: F, t4811: F, t6975: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t17032 = t6684 * t1636;
    let t17033 = t17031 * t17032;
    let t17034 = t5182 * t17033;
    let t17036 = t6689 * t1636;
    let t17037 = t10426 * t17036;
    let t17038 = t5182 * t17037;
    let t17040 = t2364 * t5068;
    let t17041 = t10426 * t17040;
    let t17042 = t5182 * t17041;
    let t17044 = t5183 * t1801;
    let t17045 = t6763 * t5186;
    let t17046 = t17044 * t17045;
    let t17047 = t5182 * t17046;
    let t17049 = t6758 * t5186;
    let t17050 = t17044 * t17049;
    let t17051 = t6674 * t17050;
    let t17054 = t4811 * t6975;
    (t17032, t17034, t17036, t17038, t17040, t17042, t17045, t17047, t17049, t17051, t17054)
}
