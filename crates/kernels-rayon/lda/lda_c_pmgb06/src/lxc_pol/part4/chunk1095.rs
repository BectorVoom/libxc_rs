//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1095/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1095(t1423: f64, t5203: f64, t1995: f64, t3223: f64, t1981: f64, t835: f64, t1461: f64, t1835: f64, t1636: f64, t1848: f64, t2880: f64, t831: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13119 = t1423 * t5203;
    let t13139 = t3223 * t1995;
    let t13177 = t1981 * t835;
    let t13182 = t1461 * t1835;
    let t13192 = t1848 * t1636;
    let t13194 = t831 * t2880;
    (t13119, t13139, t13177, t13182, t13192, t13194)
}
