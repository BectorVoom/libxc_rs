//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1079/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1079(t2149: f64, t7260: f64, t93: f64, t1805: f64, t2759: f64, t11679: f64, t68: f64, t1800: f64, t2813: f64, t6253: f64, t2854: f64, t305: f64) -> (f64, f64, f64, f64, f64) {
    let t11766 = t7260 * t2149;
    let t11767 = t93 * t11766;
    let t11773 = t2759 * t1805;
    let t11775 = t11679 * t68;
    let t11776 = t11775 * t1800;
    let t11778 = t2813 * t6253;
    let t11782 = t2854 * t305;
    (t11767, t11773, t11776, t11778, t11782)
}
