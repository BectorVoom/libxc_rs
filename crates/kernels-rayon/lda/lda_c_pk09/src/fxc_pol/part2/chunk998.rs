//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 998/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk998(t2531: f64, t5569: f64, t1336: f64, t2636: f64, t1625: f64, t2621: f64, t741: f64, t623: f64, t1397: f64, t2520: f64, t1240: f64, t93: f64) -> (f64, f64, f64, f64, f64) {
    let t10774 = t2531 * t5569;
    let t10776 = t2636 * t1336;
    let t10777 = t10776 * t1625;
    let t10779 = t741 * t2621;
    let t10780 = t10779 * t623;
    let t10786 = t2520 * t1397;
    let t10790 = t2520 * t1240;
    let t10791 = t93 * t10790;
    (t10774, t10777, t10780, t10786, t10791)
}
