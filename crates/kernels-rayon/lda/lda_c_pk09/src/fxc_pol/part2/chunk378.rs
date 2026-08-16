//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 378/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk378(t1837: f64, t93: f64, t1836: f64, t10: f64, t429: f64) -> (f64, f64, f64) {
    let t1838 = t93 * t1837;
    let t1840 = 7.108175748183851_f64 * t1836 * t1838;
    let t1841 = t429 * t10;
    (t1838, t1840, t1841)
}
