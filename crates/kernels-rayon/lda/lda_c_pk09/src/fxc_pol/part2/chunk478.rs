//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 478/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk478(t1363: f64, t2648: f64, t310: f64, t1244: f64, t1256: f64, t1264: f64, t1273: f64, t2502: f64, t2505: f64, t2542: f64, t2546: f64, t1278: f64) -> (f64, f64, f64, f64) {
    let t2649 = t2648 * t1363;
    let t2650 = t310 * t2649;
    let t2665 = t1244 - 3.2084841915276807_f64 * t2542 + t1256 + 3.2084841915276807_f64 * t2546 + t1264 - 0.64_f64 * t2502 + t1273 + 0.64_f64 * t2505;
    let t2666 = t2665 * t1278;
    (t2649, t2650, t2665, t2666)
}
