//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 982/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk982(t4790: f64, t831: f64, t2489: f64, t3223: f64, t1592: f64, t6225: f64, t495: f64, t6831: f64, t132: f64, t1547: f64, t2649: f64, t497: f64, t6904: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16743 = t831 * t4790;
    let t16749 = t3223 * t2489;
    let t16776 = t1592 * t6225;
    let t16794 = t495 * t6831;
    let t16799 = t132 * t1547 * t2649;
    let t16856 = t6904 * t497;
    (t16743, t16749, t16776, t16794, t16799, t16856)
}
