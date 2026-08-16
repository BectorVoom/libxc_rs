//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 935/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk935(t1438: f64, t154: f64, t3098: f64, t465: f64, t12514: f64, t495: f64, t5065: f64, t12535: f64, t1435: f64, t5075: f64, t3259: f64, t5066: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12991 = t154 * t1438;
    let t12995 = t154 * t3098;
    let t13000 = t465 * t3098;
    let t13007 = t5065 * t12514 * t495;
    let t13020 = t5075 * t12535 * t1435;
    let t13026 = t5075 * t5066 * t3259;
    (t12991, t12995, t13000, t13007, t13020, t13026)
}
