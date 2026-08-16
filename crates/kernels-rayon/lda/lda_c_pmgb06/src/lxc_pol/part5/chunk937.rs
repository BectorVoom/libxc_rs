//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 937/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk937(t13087: f64, t132: f64, t2851: f64, t823: f64, t1995: f64, t3223: f64, t1981: f64, t835: f64, t1461: f64, t1835: f64, t1902: f64, t3213: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13088 = t13087 / 45.0_f64;
    let t13090 = t132 * t2851 * t823;
    let t13139 = t3223 * t1995;
    let t13140 = 2.0_f64 / 45.0_f64 * t13139;
    let t13177 = t1981 * t835;
    let t13182 = t1461 * t1835;
    let t13243 = t3213 * t1902;
    (t13088, t13090, t13140, t13177, t13182, t13243)
}
