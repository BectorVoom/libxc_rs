//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 647/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk647(t5604: f64, t93: f64, t5603: f64, t1240: f64, t741: f64, t623: f64, t1470: f64, t1386: f64, t1475: f64, t1468: f64, t392: f64, t1387: f64) -> (f64, f64, f64, f64) {
    let t5605 = t93 * t5604;
    let t5606 = t5603 * t5605;
    let t5608 = t741 * t1240;
    let t5609 = t5608 * t623;
    let t5610 = t93 * t5609;
    let t5611 = t1470 * t5610;
    let t5613 = t1386 * t1475;
    let t5623 = t392 * t1468;
    let t5624 = t5623 * t1387;
    (t5606, t5611, t5613, t5624)
}
