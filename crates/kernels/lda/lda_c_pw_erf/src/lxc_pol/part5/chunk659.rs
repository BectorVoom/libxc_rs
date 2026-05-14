//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 659/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk659<F: Float>(t3661: F, t3664: F, t3764: F, t3785: F, t4562: F, t4565: F, t4569: F, t4572: F, t2437: F, t494: F, t1326: F, t1325: F, t542: F, t1313: F, t519: F, t1251: F, t2329: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6312 = 8.0 / 135.0 * t3661;
    let t6313 = 4.0 / 135.0 * t3664;
    let t6316 = 8.0 / 405.0 * t3764;
    let t6317 = 8.0 / 405.0 * t3785;
    let t6318 = 8.0 / 135.0 * t4562;
    let t6319 = 16.0 / 135.0 * t4565;
    let t6320 = 16.0 / 45.0 * t4569;
    let t6321 = 16.0 / 135.0 * t4572;
    let t6322 = t2437 * t494;
    let t6323 = t1326 * t6322;
    let t6325 = 8.0 / 45.0 * t1325 * t6323;
    let t6326 = t2437 * t542;
    let t6327 = t1313 * t6326;
    let t6329 = 4.0 / 45.0 * t519 * t6327;
    let t6330 = t1251 * t2329;
    (t6312, t6313, t6316, t6317, t6318, t6319, t6320, t6321, t6322, t6323, t6325, t6326, t6327, t6329, t6330)
}
