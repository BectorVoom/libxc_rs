//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 617/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk617(t1869: f64, t4641: f64, t1: f64, t1438: f64, t1531: f64, t10: f64, t15: f64, t1959: f64, t607: f64, t1710: f64, t883: f64, t1447: f64, t1912: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4642 = t4641 * t1869;
    let t4654 = t1438 * t1;
    let t4667 = t1531 * t1;
    let t4687 = t10 * t1;
    let t4700 = t15 * t1;
    let t4717 = 4.0_f64 / 45.0_f64 * t1959 * t607;
    let t4718 = t883 * t1710;
    let t4721 = 4.0_f64 / 135.0_f64 * t1447 * t1912;
    (t4642, t4654, t4667, t4687, t4700, t4717, t4718, t4721)
}
