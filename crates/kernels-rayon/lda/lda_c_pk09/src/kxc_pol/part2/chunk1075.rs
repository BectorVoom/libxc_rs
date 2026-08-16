//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1075/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1075(t11679: f64, t462: f64, t2042: f64, t2149: f64, t309: f64, t6611: f64, t463: f64, t453: f64, t472: f64, t2796: f64, t6253: f64, t11092: f64, t7312: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11690 = t462 * t11679;
    let t11691 = t11690 * t2042;
    let t11699 = t309 * t6611 * t2149;
    let t11700 = t463 * t11699;
    let t11702 = t453 * t11699;
    let t11704 = t472 * t11699;
    let t11706 = t2796 * t6253;
    let t11708 = t7312 * t11092;
    (t11691, t11700, t11702, t11704, t11706, t11708)
}
