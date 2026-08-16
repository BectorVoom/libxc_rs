//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1030/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1030(t11059: f64, t501: f64, t524: f64, t305: f64, t68: f64, t11092: f64, t1798: f64, t1240: f64, t2889: f64, t6267: f64, t93: f64, t1729: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11122 = t501 * t11059;
    let t11125 = t524 * t11059;
    let t11128 = t305 * t11059;
    let t11129 = t11128 * t68;
    let t11134 = t1798 * t11092;
    let t11140 = t2889 * t1240;
    let t11142 = t6267 * t93 * t11140;
    let t11144 = t2889 * t1729;
    (t11122, t11125, t11128, t11129, t11134, t11142, t11144)
}
