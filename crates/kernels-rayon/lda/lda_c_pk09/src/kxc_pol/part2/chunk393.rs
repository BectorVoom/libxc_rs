//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 393/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk393(t1800: f64, t1927: f64, t337: f64, t506: f64, t1747: f64) -> (f64, f64, f64) {
    let t1929 = 18.635258017632964_f64 * t1927 * t1800;
    let t1930 = t506 * t337;
    let t1931 = t1930 * t1747;
    (t1929, t1930, t1931)
}
