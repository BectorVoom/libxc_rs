//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 682/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk682(t1525: f64, t6146: f64, t36: f64, t3090: f64, t6151: f64, t6155: f64, t1830: f64, t2570: f64, t332: f64, t453: f64, t1: f64, t1863: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6176 = t1525 * t6146;
    let t6177 = t36 * t6176;
    let t6179 = t3090 * t6151;
    let t6180 = t36 * t6179;
    let t6182 = t1525 * t6155;
    let t6183 = t1830 * t6182;
    let t6185 = t2570 * t332;
    let t6186 = t453 * t6185;
    let t6187 = t36 * t6186;
    let t6189 = t1863 * t1;
    (t6176, t6177, t6179, t6180, t6182, t6183, t6185, t6186, t6187, t6189)
}
