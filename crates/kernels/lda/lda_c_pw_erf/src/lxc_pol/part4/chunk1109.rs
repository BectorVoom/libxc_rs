//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1109/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1109<F: Float>(t2171: F, t5251: F, t5257: F, t3794: F, t6323: F, t1325: F, t1326: F, t494: F, t6557: F, t2437: F, t944: F, t16148: F, t16152: F, t16157: F, t16160: F, t16162: F, t16164: F, t16169: F, t16171: F, t16173: F, t16175: F, t16177: F, t16179: F) -> (F, F, F, F, F, F) {
    let t16181 = 64.0 / 81.0 * t2171 * t5251;
    let t16183 = 32.0 / 27.0 * t2171 * t5257;
    let t16185 = 16.0 / 45.0 * t3794 * t6323;
    let t16189 = 16.0 / 45.0 * t1325 * t1326 * t6557 * t494;
    let t16193 = 8.0 / 45.0 * t1325 * t1326 * t2437 * t944;
    let t16194 = t16148 + t16152 + t16157 + t16160 + t16162 + t16164 + t16169 - t16171 - t16173 - t16175 + t16177 + t16179 + t16181 + t16183 + t16185 + t16189 + t16193;
    (t16181, t16183, t16185, t16189, t16193, t16194)
}
