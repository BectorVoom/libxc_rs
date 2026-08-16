//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1369/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1369(t1420: f64, t6259: f64, t493: f64, t5179: f64, t6286: f64, t6527: f64, t2485: f64, t3220: f64, t1423: f64, t6250: f64, t17950: f64, t17952: f64, t17954: f64, t17958: f64, t17961: f64, t17962: f64, t17963: f64, t17968: f64, t17971: f64, t17973: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17975 = 4.0_f64 / 15.0_f64 * t1420 * t6259;
    let t17978 = 2.0_f64 / 5.0_f64 * t493 * t5179 * t6286;
    let t17981 = 4.0_f64 / 15.0_f64 * t493 * t5179 * t6527;
    let t17982 = t3220 * t2485;
    let t17983 = 4.0_f64 / 81.0_f64 * t17982;
    let t17984 = t1423 * t6250;
    let t17985 = 4.0_f64 / 81.0_f64 * t17984;
    let t17986 = -t17950 - t17952 - t17954 - t17958 - t17961 + t17962 - t17963 + t17968 + t17971 - t17973 + t17975 - t17978 + t17981 + t17983 + t17985;
    (t17975, t17978, t17981, t17983, t17985, t17986)
}
