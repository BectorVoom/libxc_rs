//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1183/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1183(t21337: f64, t21356: f64, t38: f64, t56: f64, t18585: f64, t18589: f64, t18615: f64, t110: f64, t7321: f64, t360: f64, t7317: f64, t350: f64, t365: f64, t7278: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21358 = t21337 / 2.0_f64 + t21356 / 2.0_f64;
    let t21361 = 2.923025_f64 * t38 * t56 * t21358;
    let t21366 = 8.769075_f64 * t18585;
    let t21367 = 5.84605_f64 * t18589;
    let t21369 = 2.923025_f64 * t18615;
    let t21375 = t110 * t7321;
    let t21376 = t360 * t21375;
    let t21378 = t110 * t7317;
    let t21379 = t360 * t21378;
    let t21382 = t365 * t7278 * t350;
    (t21358, t21361, t21366, t21367, t21369, t21375, t21376, t21378, t21379, t21382)
}
