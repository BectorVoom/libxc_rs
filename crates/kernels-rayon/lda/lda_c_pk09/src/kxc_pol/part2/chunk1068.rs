//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1068/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1068(t11551: f64, t11577: f64, t447: f64, t452: f64, t1971: f64, t2846: f64, t2794: f64, t7284: f64, t7286: f64, t446: f64, t95: f64, t1815: f64) -> (f64, f64, f64, f64, f64) {
    let t11578 = t11551 + t11577;
    let t11579 = t447 * t11578;
    let t11580 = t11579 * t452;
    let t11583 = t2846 * t1971;
    let t11586 = t2794 * t7284;
    let t11587 = t11586 * t7286;
    let t11588 = t95 * t446;
    let t11589 = t11588 * t1815;
    (t11580, t11583, t11586, t11587, t11589)
}
