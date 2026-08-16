//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 668/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk668(t1853: f64, t6196: f64, t1777: f64, t1947: f64, t2042: f64, t1931: f64, t1943: f64, t1240: f64, t1906: f64, t1905: f64, t1948: f64, t1920: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6197 = t1853 * t6196;
    let t6199 = t1777 * t1947;
    let t6200 = t6199 * t2042;
    let t6210 = t1931 * t6196;
    let t6212 = t1943 * t1947;
    let t6213 = t6212 * t2042;
    let t6215 = t1906 * t1240;
    let t6216 = t1905 * t6215;
    let t6217 = t1948 * t6216;
    let t6223 = t1920 * t1240;
    (t6197, t6200, t6210, t6213, t6217, t6223)
}
