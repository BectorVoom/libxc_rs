//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 443/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk443(t110: f64, t2335: f64, t89: f64, t2152: f64, t96: f64, t93: f64) -> (f64, f64, f64, f64) {
    let t2336 = t110 * t2335;
    let t2337 = t2336 * t89;
    let t2340 = t96 * t2152;
    let t2341 = t93 * t2340;
    (t2336, t2337, t2340, t2341)
}
