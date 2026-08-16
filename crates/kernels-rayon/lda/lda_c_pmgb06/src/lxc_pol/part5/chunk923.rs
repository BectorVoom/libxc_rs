//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 923/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk923(t10318: f64, t806: f64, t1600: f64, t1988: f64, t1898: f64, t3213: f64, t161: f64, t3004: f64, t843: f64, t132: f64, t1547: f64, t2065: f64) -> (f64, f64, f64, f64, f64) {
    let t11866 = t10318 * t806;
    let t11867 = 2.0_f64 / 135.0_f64 * t11866;
    let t11877 = t1988 * t1600;
    let t11881 = t3213 * t1898;
    let t11882 = 4.0_f64 / 135.0_f64 * t11881;
    let t11884 = t161 * t3004 * t843;
    let t11897 = t132 * t1547 * t2065;
    (t11867, t11877, t11882, t11884, t11897)
}
