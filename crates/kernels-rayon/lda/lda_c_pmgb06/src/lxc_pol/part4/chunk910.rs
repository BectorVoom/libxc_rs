//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 910/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk910(t6624: f64, t2605: f64, t435: f64, t132: f64, t337: f64, t6560: f64, t5069: f64, t5068: f64, t5139: f64, t5138: f64, t1593: f64, t443: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6625 = 2.0_f64 / 45.0_f64 * t6624;
    let t6626 = t435 * t2605;
    let t6627 = t132 * t6626;
    let t6628 = 2.0_f64 / 45.0_f64 * t6627;
    let t6629 = t6560 * t337;
    let t6630 = t5069 * t6629;
    let t6632 = 4.0_f64 / 45.0_f64 * t5068 * t6630;
    let t6633 = t5139 * t6629;
    let t6635 = 2.0_f64 / 27.0_f64 * t5138 * t6633;
    let t6636 = t1593 * t443;
    (t6625, t6626, t6628, t6629, t6630, t6632, t6633, t6635, t6636)
}
