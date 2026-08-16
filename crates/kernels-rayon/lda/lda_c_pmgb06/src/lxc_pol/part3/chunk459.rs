//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 459/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk459(t1868: f64, t453: f64, t1830: f64, t1522: f64, t1523: f64, t1856: f64, t1861: f64, t1866: f64) -> (f64, f64, f64) {
    let t1869 = t453 * t1868;
    let t1870 = t1830 * t1869;
    let t1872 = -t1522 - 0.0006297222222222223_f64 * t1523 - 0.0006297222222222223_f64 * t1856 + 0.0012594444444444445_f64 * t1861 - 0.003778333333333333_f64 * t1866 + 0.003778333333333333_f64 * t1870;
    (t1869, t1870, t1872)
}
