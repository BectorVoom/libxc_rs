//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 664/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk664<F: Float>(t242: F, t4100: F, t1198: F, t632: F, t1143: F, t458: F, t1203: F, t1155: F, t1726: F, t405: F, t1112: F, t462: F, t159: F, t285: F, t1159: F, t477: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4101 = t4100 * t242;
    let t4103 = t1198 * t632;
    let t4106 = 0.2512884616065132 * t458 * t1143;
    let t4110 = t1203 * t632;
    let t4113 = 0.5025769232130264 * t1155 * t242;
    let t4117 = t405 * t1726;
    let t4120 = t462 * t1112;
    let t4122 = t4120 * t159 * t285;
    let t4125 = t1159 * t477 * t285;
    (t4101, t4103, t4106, t4110, t4113, t4117, t4120, t4122, t4125)
}
