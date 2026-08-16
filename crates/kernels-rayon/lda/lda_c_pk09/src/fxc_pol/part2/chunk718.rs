//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 718/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk718(t1672: f64, t2091: f64, t2088: f64, t451: f64, t6700: f64, t2042: f64, t1947: f64, t2084: f64, t2083: f64, t305: f64, t462: f64, t2070: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7230 = t2091 * t1672;
    let t7232 = t2088 * t1672;
    let t7240 = t451 * t6700;
    let t7241 = t7240 * t2042;
    let t7243 = t2084 * t1947;
    let t7244 = t7243 * t2042;
    let t7248 = t2083 * t305;
    let t7252 = t462 * t6700;
    let t7253 = t7252 * t2042;
    let t7255 = t2070 * t1947;
    (t7230, t7232, t7241, t7244, t7248, t7253, t7255)
}
