//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 882/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk882<F: Float>(t2527: F, t642: F, t5068: F, t10365: F, t15891: F, t1899: F, t5191: F, t5194: F, t6763: F, t5182: F, t140: F, t5180: F, t5598: F, t1757: F, t220: F, t5193: F) -> (F, F, F, F, F, F, F, F) {
    let t15892 = t642 * t2527;
    let t15893 = t15892 * t5068;
    let t15894 = t10365 * t15893;
    let t15895 = t15891 * t15894;
    let t15897 = t5191 * t1899;
    let t15898 = t6763 * t5194;
    let t15899 = t15897 * t15898;
    let t15900 = t5182 * t15899;
    let t15903 = t140 * t5598 * t5180;
    let t15904 = t220 * t1757;
    let t15905 = t5193 * t15904;
    (t15892, t15893, t15895, t15897, t15898, t15900, t15903, t15905)
}
