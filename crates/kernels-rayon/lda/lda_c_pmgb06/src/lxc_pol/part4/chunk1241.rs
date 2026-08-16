//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1241/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1241(t16343: f64, t446: f64, t1427: f64, t6127: f64, t1989: f64, t5305: f64, t2493: f64, t3213: f64, t1963: f64, t5187: f64, t1083: f64, t6502: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16345 = 2.0_f64 / 45.0_f64 * t16343 * t446;
    let t16347 = 2.0_f64 / 45.0_f64 * t6127 * t1427;
    let t16349 = 4.0_f64 / 45.0_f64 * t5305 * t1989;
    let t16350 = t3213 * t2493;
    let t16351 = 4.0_f64 / 405.0_f64 * t16350;
    let t16353 = 4.0_f64 / 45.0_f64 * t5187 * t1963;
    let t16354 = t6502 * t1083;
    (t16345, t16347, t16349, t16351, t16353, t16354)
}
