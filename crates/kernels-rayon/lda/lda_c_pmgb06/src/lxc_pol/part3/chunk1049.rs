//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1049/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1049(t2924: f64, t5138: f64, t852: f64, t2992: f64, t5090: f64, t1586: f64, t764: f64, t5068: f64, t529: f64, t6559: f64, t337: f64, t5069: f64) -> (f64, f64, f64, f64, f64) {
    let t12476 = t5138 * t852 * t2924 / 9.0_f64;
    let t12479 = 2.0_f64 / 9.0_f64 * t5138 * t5090 * t2992;
    let t12480 = t764 * t1586;
    let t12484 = 2.0_f64 / 15.0_f64 * t5068 * t6559 * t12480 * t529;
    let t12485 = t12480 * t337;
    let t12488 = 2.0_f64 / 15.0_f64 * t5068 * t5069 * t12485;
    (t12476, t12479, t12484, t12485, t12488)
}
