//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 598/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk598(t122: f64, t227: f64, t4182: f64, t1135: f64, t199: f64, t2790: f64, t2837: f64, t29: f64, t563: f64, t1187: f64, t1190: f64, t2841: f64, t98: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4185 = 0.19455129084526285_f64 * t122 * t4182 * t227;
    let t4187 = 0.5025769232130264_f64 * t1135 * t199;
    let t4188 = t2790 / 2.0_f64;
    let t4189 = t2837 * t29;
    let t4191 = 0.09405_f64 * t4189 * t563;
    let t4193 = 0.1254_f64 * t1187 * t1190;
    let t4194 = t2841 * t98;
    (t4185, t4187, t4188, t4189, t4191, t4193, t4194)
}
