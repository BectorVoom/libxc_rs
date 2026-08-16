//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 674/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk674(t1927: f64, t6292: f64, t1468: f64, t496: f64, t1747: f64, t4993: f64, t95: f64, t333: f64) -> (f64, f64, f64) {
    let t6294 = 18.635258017632964_f64 * t1927 * t6292;
    let t6299 = t496 * t1468;
    let t6300 = t6299 * t1747;
    let t6301 = t95 * t4993;
    let t6302 = t333 * t6301;
    (t6294, t6300, t6302)
}
