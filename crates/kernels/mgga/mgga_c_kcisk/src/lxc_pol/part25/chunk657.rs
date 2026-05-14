//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 657/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk657<F: Float>(t1895: F, t6974: F, t1869: F, t1900: F, t6719: F, t1636: F, t2571: F, t5192: F, t5182: F, t1894: F, t2063: F, t5185: F, t5184: F, t1333: F, t2534: F, t2510: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6975 = t6974 * t1895;
    let t6976 = t1869 * t6975;
    let t6978 = t6719 * t1900;
    let t6979 = t1869 * t6978;
    let t6981 = t2571 * t1636;
    let t6982 = t5192 * t6981;
    let t6983 = t5182 * t6982;
    let t6985 = t2063 * t1894;
    let t6986 = t5185 * t6985;
    let t6987 = t5184 * t6986;
    let t6988 = t5182 * t6987;
    let t6990 = t1333 * t2534;
    let t6992 = t1333 * t2510;
    (t6975, t6976, t6978, t6979, t6981, t6982, t6983, t6986, t6987, t6988, t6990, t6992)
}
