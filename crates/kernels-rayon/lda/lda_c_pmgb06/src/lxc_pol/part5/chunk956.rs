//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 956/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk956(t301: f64, t413: f64, t5988: f64, t5980: f64, t76: f64, t123: f64, t317: f64, t6104: f64, t740: f64, t73: f64, t1122: f64, t2395: f64, t30: f64) -> (f64, f64, f64, f64, f64) {
    let t14789 = t5988 * t413 * t301;
    let t14797 = t76 * t5980;
    let t14852 = t123 * t740 * t6104 * t317;
    let t14875 = t73 * t5980;
    let t14939 = t2395 * t30 * t1122;
    (t14789, t14797, t14852, t14875, t14939)
}
