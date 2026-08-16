//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 655/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk655(t1795: f64, t415: f64, t117: f64, t123: f64, t740: f64, t859: f64, t2209: f64, t73: f64, t1282: f64, t2229: f64, t365: f64, t110: f64, t30: f64, t342: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5702 = t1795 * t415;
    let t5712 = t123 * t740 * t859 * t117;
    let t5721 = t73 * t2209;
    let t5740 = t1282 * t2209;
    let t5770 = t365 * t2229;
    let t5772 = t30 * t110 * t342;
    (t5702, t5712, t5721, t5740, t5770, t5772)
}
