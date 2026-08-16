//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 936/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk936(t154: f64, t3092: f64, t465: f64, t12535: f64, t441: f64, t5075: f64, t1464: f64, t1601: f64, t2918: f64, t518: f64, t1554: f64, t161: f64, t2089: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13027 = t154 * t3092;
    let t13031 = t465 * t3092;
    let t13043 = t5075 * t12535 * t441;
    let t13064 = t1601 * t1464;
    let t13068 = t518 * t2918;
    let t13087 = t161 * t1554 * t2089;
    (t13027, t13031, t13043, t13064, t13068, t13087)
}
