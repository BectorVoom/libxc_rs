//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 648/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk648(t1397: f64, t1471: f64, t5624: f64, t93: f64, t334: f64, t5031: f64, t1216: f64, t1458: f64, t1435: f64, t1538: f64, t1510: f64, t1431: f64, t1507: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5625 = t1471 * t1397;
    let t5627 = t5624 * t93 * t5625;
    let t5632 = t5031 * t334;
    let t5635 = t1216 * t1458;
    let t5637 = t1538 * t1435;
    let t5639 = t1510 * t1435;
    let t5641 = t1507 * t1431;
    (t5627, t5632, t5635, t5637, t5639, t5641)
}
