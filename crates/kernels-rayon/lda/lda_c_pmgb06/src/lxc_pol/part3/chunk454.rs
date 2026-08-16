//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 454/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk454(t1831: f64, t506: f64, t1830: f64, t1473: f64, t1474: f64, t1818: f64, t1823: f64, t1828: f64) -> (f64, f64, f64) {
    let t1832 = t506 * t1831;
    let t1833 = t1830 * t1832;
    let t1835 = -t1473 - 0.0006297222222222223_f64 * t1474 - 0.0006297222222222223_f64 * t1818 + 0.0012594444444444445_f64 * t1823 - 0.003778333333333333_f64 * t1828 - 0.003778333333333333_f64 * t1833;
    (t1832, t1833, t1835)
}
