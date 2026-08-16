//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 920/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk920(t27: f64, t2789: f64, t29: f64, t563: f64, t115: f64, t2786: f64, t562: f64, t1190: f64, t4189: f64, t1187: f64, t4197: f64, t8173: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10512 = t2789 * t27;
    let t10515 = 0.1254_f64 * t10512 * t29 * t563;
    let t10518 = 0.32511111111111113_f64 * t562 * t2786 * t115;
    let t10520 = 0.2508_f64 * t4189 * t1190;
    let t10522 = 0.39013333333333333_f64 * t1187 * t4197;
    let t10524 = t8173 * t115;
    (t10512, t10515, t10518, t10520, t10522, t10524)
}
