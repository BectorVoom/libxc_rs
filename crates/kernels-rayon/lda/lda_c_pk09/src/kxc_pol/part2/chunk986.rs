//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 986/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk986(t10555: f64, t10571: f64, t10533: f64, t10535: f64, t10540: f64, t1397: f64, t1417: f64, t2621: f64, t392: f64, t5139: f64, t5144: f64, t1425: f64) -> (f64, f64) {
    let t10572 = t10555 + t10571;
    let t10575 = t10533 * t392 - t10535 * t1397 / 2.0_f64 - t5139 * t2621 / 2.0_f64 + 3.0_f64 / 4.0_f64 * t5144 * t10540 - t1417 * t10572 / 2.0_f64;
    let t10576 = t10575 * t1425;
    (t10572, t10576)
}
