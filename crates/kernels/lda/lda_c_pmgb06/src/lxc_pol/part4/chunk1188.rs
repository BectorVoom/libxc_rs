//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1188/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1188<F: Float>(t493: F, t5179: F, t6113: F, t1420: F, t6255: F, t6259: F, t6286: F, t6527: F, t2485: F, t3220: F, t1423: F, t6250: F, t17950: F, t17952: F, t17954: F, t17958: F, t17961: F, t17962: F, t17963: F, t17968: F) -> (F, F, F, F, F, F, F, F) {
    let t17971 = 2.0 / 15.0 * t493 * t5179 * t6113;
    let t17973 = 2.0 / 5.0 * t1420 * t6255;
    let t17975 = 4.0 / 15.0 * t1420 * t6259;
    let t17978 = 2.0 / 5.0 * t493 * t5179 * t6286;
    let t17981 = 4.0 / 15.0 * t493 * t5179 * t6527;
    let t17982 = t3220 * t2485;
    let t17983 = 4.0 / 81.0 * t17982;
    let t17984 = t1423 * t6250;
    let t17985 = 4.0 / 81.0 * t17984;
    let t17986 = -t17950 - t17952 - t17954 - t17958 - t17961 + t17962 - t17963 + t17968 + t17971 - t17973 + t17975 - t17978 + t17981 + t17983 + t17985;
    (t17971, t17973, t17975, t17978, t17981, t17983, t17985, t17986)
}
