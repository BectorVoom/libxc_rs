//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 389/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk389(t1864: f64, t453: f64, t36: f64, t1: f64, t443: f64, t1830: f64, t1522: f64, t1523: f64, t1856: f64, t1861: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1865 = t453 * t1864;
    let t1866 = t36 * t1865;
    let t1868 = t443 * t1;
    let t1869 = t453 * t1868;
    let t1870 = t1830 * t1869;
    let t1872 = -t1522 - 0.0006297222222222223_f64 * t1523 - 0.0006297222222222223_f64 * t1856 + 0.0012594444444444445_f64 * t1861 - 0.003778333333333333_f64 * t1866 + 0.003778333333333333_f64 * t1870;
    (t1865, t1866, t1868, t1869, t1870, t1872)
}
