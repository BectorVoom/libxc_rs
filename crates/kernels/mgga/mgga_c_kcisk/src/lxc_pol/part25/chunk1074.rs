//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1074/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1074<F: Float>(t31966: F, t856: F, t2679: F, t15445: F, t2676: F, t2927: F, t9310: F, t3174: F, t981: F, t140: F, t178: F, t9331: F, t937: F, t975: F, t3137: F, t3139: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t31967 = t856 * t31966;
    let t31968 = t31967 * t2679;
    let t31970 = t15445 * t2676;
    let t31971 = t31970 * t2679;
    let t31973 = t2927 * t9310;
    let t31974 = t31973 * t2679;
    let t31976 = t981 * t3174;
    let t31978 = t140 * t178 * t31976;
    let t31981 = t140 * t937 * t9331;
    let t31984 = t140 * t975 * t9331;
    let t31986 = t3137 * t3139;
    (t31967, t31968, t31970, t31971, t31973, t31974, t31976, t31978, t31981, t31984, t31986)
}
