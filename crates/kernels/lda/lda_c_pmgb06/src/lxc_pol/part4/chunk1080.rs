//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1080/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1080<F: Float>(t13712: F, t806: F, t1436: F, t1439: F, t2010: F, t760: F, t2485: F, t3213: F, t2481: F, t1415: F, t1981: F, t496: F, t764: F, t2493: F, t3220: F, t132: F, t1547: F, t2605: F) -> (F, F, F, F, F, F, F) {
    let t16144 = t13712 * t806;
    let t16145 = 4.0 / 405.0 * t16144;
    let t16149 = 4.0 / 27.0 * t2010 * t1436 * t1439 * t760;
    let t16150 = t3213 * t2485;
    let t16151 = 2.0 / 243.0 * t16150;
    let t16152 = t3213 * t2481;
    let t16153 = 2.0 / 405.0 * t16152;
    let t16157 = 8.0 / 45.0 * t1981 * t496 * t1415 * t764;
    let t16158 = t3220 * t2493;
    let t16159 = 8.0 / 135.0 * t16158;
    let t16161 = t132 * t1547 * t2605;
    (t16145, t16149, t16151, t16153, t16157, t16159, t16161)
}
