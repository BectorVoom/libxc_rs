//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 691/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk691(t6522: f64, t1686: f64, t5153: f64, t1240: f64, t633: f64, t6511: f64) -> (f64, f64, f64, f64) {
    let t6523 = 4.277978922036907_f64 * t6522;
    let t6524 = t1686 * t5153;
    let t6525 = t1240 * t633;
    let t6527 = t6524 * t6511 * t6525;
    (t6523, t6524, t6525, t6527)
}
