//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 908/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk908(t166: f64, t6595: f64, t161: f64, t4815: f64, t822: f64, t137: f64, t132: f64, t1848: f64, t853: f64, t2101: f64, t831: f64, t5349: f64, t5354: f64, t5356: f64, t5363: f64, t5369: f64, t5370: f64, t5372: f64, t6586: f64, t6588: f64, t6590: f64, t6592: f64, t6594: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6596 = t166 * t6595;
    let t6598 = t161 * t6596 / 30.0_f64;
    let t6599 = t4815 * t822;
    let t6600 = t137 * t6599;
    let t6602 = t132 * t6600 / 15.0_f64;
    let t6604 = t1848 * t853 / 15.0_f64;
    let t6606 = t831 * t2101 / 15.0_f64;
    let t6607 = -t5349 + t5354 - t5356 - t5363 + t5369 + 8.0_f64 / 9.0_f64 * t5370 - 4.0_f64 / 27.0_f64 * t5372 - t6586 - t6588 - t6590 - t6592 - t6594 - t6598 - t6602 - t6604 - t6606;
    (t6596, t6598, t6599, t6600, t6602, t6604, t6606, t6607)
}
