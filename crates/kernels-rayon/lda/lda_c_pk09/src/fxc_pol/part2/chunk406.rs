//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 406/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk406(t2040: f64, t2042: f64, t1947: f64, t471: f64, t305: f64, t450: f64, t1819: f64, t462: f64, t1672: f64, t463: f64, t1754: f64, t1765: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2044 = t2040 * t2042 / 6.0_f64;
    let t2045 = t471 * t1947;
    let t2047 = t2045 * t2042 / 6.0_f64;
    let t2052 = t450 * t305;
    let t2053 = t1819 * t2052;
    let t2056 = t462 * t1947;
    let t2058 = t2056 * t2042 / 6.0_f64;
    let t2060 = t463 * t1672 / 18.0_f64;
    let t2061 = 1.5323028051206833_f64 * t1754;
    let t2063 = 0.5107676017068944_f64 * t1765;
    (t2044, t2045, t2047, t2052, t2053, t2056, t2058, t2060, t2061, t2063)
}
