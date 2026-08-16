//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 608/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk608(t1: f64, t1219: f64, t1322: f64, t2257: f64, t384: f64, t769: f64, t1152: f64, t123: f64, t868: f64, t740: f64, t794: f64) -> (f64, f64, f64, f64, f64) {
    let t4381 = t1219 * t1;
    let t4398 = t2257 * t1322;
    let t4414 = t384 * t769;
    let t4427 = t123 * t1152 * t868;
    let t4429 = t740 * t794;
    (t4381, t4398, t4414, t4427, t4429)
}
