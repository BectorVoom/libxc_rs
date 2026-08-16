//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 408/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk408(t132: f64, t1928: f64, t436: f64, t802: f64, t489: f64, t843: f64) -> (f64, f64, f64, f64, f64) {
    let t1929 = t132 * t1928;
    let t1930 = t1929 / 45.0_f64;
    let t1931 = t802 * t436;
    let t1932 = t1931 / 45.0_f64;
    let t1933 = t489 * t843;
    (t1929, t1930, t1931, t1932, t1933)
}
