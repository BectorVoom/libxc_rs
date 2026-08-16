//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 999/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk999(t10791: f64, t1470: f64, t1214: f64, t2520: f64, t93: f64, t2143: f64, t3677: f64, t623: f64, t10020: f64, t1487: f64, t1406: f64, t9836: f64) -> (f64, f64, f64, f64, f64) {
    let t10792 = t1470 * t10791;
    let t10794 = t2520 * t1214;
    let t10795 = t93 * t10794;
    let t10798 = t3677 * t2143;
    let t10799 = t10798 * t623;
    let t10800 = t93 * t10799;
    let t10803 = t1487 * t10020;
    let t10808 = t1406 * t9836;
    (t10792, t10795, t10800, t10803, t10808)
}
