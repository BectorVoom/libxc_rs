//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1045/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1045<F: Float>(t6715: F, t6719: F, t6713: F, t1894: F, t8939: F, t1801: F, t5062: F, t1869: F, t1757: F, t1899: F, t1873: F, t17057: F, t6685: F, t2364: F, t6684: F, t17031: F) -> (F, F, F, F, F, F, F, F) {
    let t24011 = t6719 * t6715;
    let t24012 = t6713 * t24011;
    let t24014 = t8939 * t1894;
    let t24015 = t1801 * t24014;
    let t24016 = t5062 * t24015;
    let t24017 = t1869 * t24016;
    let t24019 = t8939 * t1757;
    let t24020 = t1899 * t24019;
    let t24021 = t1873 * t24020;
    let t24022 = t1869 * t24021;
    let t24024 = t17057 * t6685;
    let t24025 = t1869 * t24024;
    let t24027 = t2364 * t6684;
    let t24028 = t17031 * t24027;
    (t24012, t24014, t24017, t24019, t24022, t24025, t24027, t24028)
}
