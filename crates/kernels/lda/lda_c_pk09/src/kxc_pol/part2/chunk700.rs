//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 700/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk700<F: Float>(t4762: F, t515: F, t476: F, t1797: F, t6287: F, t1800: F, t537: F, t1926: F, t524: F, t1930: F, t507: F, t1729: F, t337: F) -> (F, F, F, F, F, F, F, F) {
    let t6670 = t4762 * t515;
    let t6672 = F::new(0.018289183791044262) * t476 * t6670;
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
