//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 948/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk948(t1101: f64, t2160: f64, t2158: f64, t2799: f64, t898: f64, t2801: f64, t3947: f64, t3952: f64, t11062: f64, t283: f64, t8647: f64, t8651: f64, t8655: f64, t8659: f64, t8663: f64, t8668: f64, t8670: f64, t8673: f64, t8675: f64, t8684: f64) -> f64 {
    let t11090 = t1101 * t2160;
    let t11092 = t1101 * t2158;
    let t11093 = 60.0_f64 * t11092;
    let t11095 = t2799 * t898;
    let t11097 = t2801 * t898;
    let t11098 = 144.0_f64 * t11097;
    let t11099 = t3947 * t898;
    let t11100 = 240.0_f64 * t11099;
    let t11101 = t3952 * t898;
    let t11104 = -t8647 - t8651 + t8655 + t8659 + 0.0007324578922402618_f64 * t8663 + t8668 - 0.00018311447306006544_f64 * t8670 + 0.0197516734986138_f64 * t11062 * t283 + 60.0_f64 * t11090 + t11093 - 0.5848223622634646_f64 * t8673 + 24.0_f64 * t11095 - t11098 + t11100 - 120.0_f64 * t11101 + 3.5089341735807875_f64 * t8675 - t8684;
    t11104
}
