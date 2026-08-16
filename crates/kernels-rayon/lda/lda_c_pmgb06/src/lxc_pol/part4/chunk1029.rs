//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1029/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1029(t10506: f64, t1195: f64, t98: f64, t4194: f64, t4200: f64, t27: f64, t2789: f64, t29: f64, t563: f64, t115: f64, t2786: f64, t562: f64) -> (f64, f64, f64, f64, f64) {
    let t10509 = 0.04717548_f64 * t10506 * t98 * t1195;
    let t10511 = 0.12580128_f64 * t4194 * t4200;
    let t10512 = t2789 * t27;
    let t10515 = 0.1254_f64 * t10512 * t29 * t563;
    let t10518 = 0.32511111111111113_f64 * t562 * t2786 * t115;
    (t10509, t10511, t10512, t10515, t10518)
}
