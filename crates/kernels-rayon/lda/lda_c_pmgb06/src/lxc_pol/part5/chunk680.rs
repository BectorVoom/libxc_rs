//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 680/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk680(t2377: f64, t3098: f64, t332: f64, t1619: f64, t3092: f64, t3404: f64, t1: f64, t1858: f64, t1531: f64, t2381: f64, t453: f64, t36: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6145 = t3098 * t2377;
    let t6146 = t6145 * t332;
    let t6147 = t1619 * t6146;
    let t6150 = t3092 * t2377;
    let t6151 = t6150 * t332;
    let t6152 = t3404 * t6151;
    let t6155 = t1858 * t1;
    let t6156 = t1619 * t6155;
    let t6159 = t1531 * t2381;
    let t6160 = t6159 * t332;
    let t6161 = t453 * t6160;
    let t6162 = t36 * t6161;
    (t6145, t6146, t6147, t6150, t6151, t6152, t6155, t6156, t6160, t6161, t6162)
}
