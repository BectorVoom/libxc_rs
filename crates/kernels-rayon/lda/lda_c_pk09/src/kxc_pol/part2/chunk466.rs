//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 466/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk466(t2143: f64, t48: f64, t285: f64, t284: f64, t1584: f64, t1586: f64, t1588: f64, t1590: f64, t2502: f64, t2505: f64, t2542: f64, t323: f64) -> (f64, f64, f64, f64) {
    let t2544 = t48 * t2143;
    let t2545 = t285 * t2544;
    let t2546 = t284 * t2545;
    let t2550 = t1584 - 1.5323028051206833_f64 * t2542 + t1586 + 1.5323028051206833_f64 * t2546 + t1588 - 0.3056501876701794_f64 * t2502 + t1590 + 0.3056501876701794_f64 * t2505;
    let t2551 = t323 * t2550;
    (t2544, t2546, t2550, t2551)
}
