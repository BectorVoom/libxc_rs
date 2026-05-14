//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 958/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk958<F: Float>(t16782: F, t16850: F, t16903: F, t16978: F, t1801: F, t1873: F, t1869: F, t4804: F, t6719: F, t11659: F, t2528: F, t4826: F, t7274: F, t1790: F, t5043: F, t7283: F) -> (F, F, F, F, F, F, F, F) {
    let t16980 = t16782 + t16850 + t16903 + t16978;
    let t16981 = t1801 * t16980;
    let t16982 = t1873 * t16981;
    let t16983 = t1869 * t16982;
    let t16985 = t6719 * t4804;
    let t16986 = t1869 * t16985;
    let t16988 = t11659 * t2528;
    let t16989 = t1869 * t16988;
    let t16991 = t7274 * t4826;
    let t16992 = t16991 * t1790;
    let t16997 = t7283 * t5043;
    (t16980, t16981, t16983, t16986, t16989, t16991, t16992, t16997)
}
