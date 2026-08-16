//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 938/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk938(t13243: f64, t1555: f64, t1848: f64, t3155: f64, t831: f64, t177: f64, t2911: f64, t2918: f64, t1531: f64, t1593: f64, t1827: f64, t947: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13244 = 2.0_f64 / 81.0_f64 * t13243;
    let t13291 = t1848 * t1555;
    let t13292 = t13291 / 45.0_f64;
    let t13294 = t831 * t3155;
    let t13295 = t13294 / 45.0_f64;
    let t13300 = t177 * t2911;
    let t13304 = t177 * t2918;
    let t13308 = t1593 * t1531;
    let t13370 = t947 * t1827;
    (t13244, t13292, t13295, t13300, t13304, t13308, t13370)
}
