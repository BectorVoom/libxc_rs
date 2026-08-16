//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1051/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1051(t1083: f64, t5070: f64, t5068: f64, t5069: f64, t2872: f64, t3458: f64, t851: f64, t2876: f64, t5090: f64, t12473: f64, t12476: f64, t12479: f64, t12484: f64, t12488: f64, t12491: f64, t12493: f64, t12496: f64, t12500: f64) -> (f64, f64, f64, f64, f64) {
    let t12501 = t5070 * t1083;
    let t12504 = 2.0_f64 / 15.0_f64 * t5068 * t5069 * t12501;
    let t12508 = 2.0_f64 / 5.0_f64 * t5068 * t3458 * t851 * t2872;
    let t12511 = 4.0_f64 / 15.0_f64 * t5068 * t5090 * t2876;
    let t12512 = t12473 - t12476 + t12479 + t12484 + t12488 - t12491 + t12493 - t12496 + t12500 + t12504 - t12508 - t12511;
    (t12501, t12504, t12508, t12511, t12512)
}
