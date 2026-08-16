//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 700/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk700(t4762: f64, t515: f64, t476: f64, t1797: f64, t6287: f64, t1800: f64, t537: f64, t1926: f64, t524: f64, t1930: f64, t507: f64, t1729: f64, t337: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6670 = t4762 * t515;
    let t6672 = 0.018289183791044262_f64 * t476 * t6670;
    let t6676 = t1797 * t6287;
    let t6677 = t6676 * t1800;
    let t6679 = t537 * t6287;
    let t6685 = t1926 * t6287;
    let t6686 = t6685 * t1800;
    let t6688 = t524 * t6287;
    let t6691 = t1930 * t6287;
    let t6692 = t6691 * t1800;
    let t6694 = t507 * t6287;
    let t6700 = t337 * t1729;
    (t6672, t6677, t6679, t6686, t6688, t6692, t6694, t6700)
}
