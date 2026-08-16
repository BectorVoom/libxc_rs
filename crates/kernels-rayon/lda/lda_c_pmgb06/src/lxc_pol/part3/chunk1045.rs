//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1045/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1045(t12431: f64, t131: f64, t155: f64, t44: f64, t460: f64, t4754: f64, t432: f64, t4682: f64, t1491: f64, t1848: f64, t12304: f64, t12307: f64, t12308: f64, t12311: f64, t12313: f64, t12315: f64, t12415: f64, t12417: f64) -> (f64, f64, f64, f64, f64) {
    let t12435 = t12431 * t44 * t131 * t155 / 30.0_f64;
    let t12437 = t4754 * t460 / 10.0_f64;
    let t12439 = t432 * t4682 / 10.0_f64;
    let t12441 = t1848 * t1491 / 10.0_f64;
    let t12442 = 2.0_f64 / 3.0_f64 * t12304 + t12307 + 2.0_f64 / 3.0_f64 * t12308 + t12311 + t12313 + t12315 + t12415 + t12417 + t12435 + t12437 + t12439 + t12441;
    (t12435, t12437, t12439, t12441, t12442)
}
