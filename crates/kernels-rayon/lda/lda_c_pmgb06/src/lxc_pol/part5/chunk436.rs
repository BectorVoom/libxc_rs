//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 436/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk436(t1831: f64, t525: f64, t103: f64, t1474: f64, t1563: f64, t1571: f64, t1572: f64, t1818: f64, t1823: f64, t1828: f64, t1833: f64, t2060: f64, t2077: f64, t2079: f64, t2082: f64) -> (f64, f64) {
    let t2085 = t525 * t1831;
    let t2088 = t1563 + 0.011997222222222222_f64 * t1474 + 0.011997222222222222_f64 * t1818 - 0.023994444444444443_f64 * t1823 + 0.07198333333333333_f64 * t1828 + 0.07198333333333333_f64 * t1833 + t1571 + 0.0044444444444444444_f64 * t1572 + 0.0044444444444444444_f64 * t2077 - 0.0022222222222222222_f64 * t103 * t2079 + 0.013333333333333334_f64 * t103 * t2082 + 0.013333333333333334_f64 * t2060 * t2085;
    (t2085, t2088)
}
