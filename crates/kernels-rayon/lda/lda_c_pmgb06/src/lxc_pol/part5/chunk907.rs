//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 907/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk907(t11065: f64, t4481: f64, t638: f64, t8614: f64, t1101: f64, t2160: f64, t2158: f64, t2799: f64, t898: f64, t2801: f64, t3947: f64, t3952: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11066 = 24.0_f64 * t11065;
    let t11073 = 24.0_f64 * t638 * t4481;
    let t11083 = 240.0_f64 * t8614;
    let t11090 = t1101 * t2160;
    let t11092 = t1101 * t2158;
    let t11093 = 60.0_f64 * t11092;
    let t11095 = t2799 * t898;
    let t11097 = t2801 * t898;
    let t11098 = 144.0_f64 * t11097;
    let t11099 = t3947 * t898;
    let t11100 = 240.0_f64 * t11099;
    let t11101 = t3952 * t898;
    (t11066, t11073, t11083, t11090, t11093, t11095, t11098, t11100, t11101)
}
