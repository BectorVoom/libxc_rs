//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 202/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk202<F: Float>(t188: F, t655: F, t659: F, t694: F, t183: F, t186: F, t89: F, t132: F, t61: F) -> (F, F, F, F, F, F, F) {
    let t697 = t655 * t188 - t659 * t694 / F::new(2.0);
    let t698 = t183 * t183;
    let t699 = F::new(1.0) / t186;
    let t701 = -t698 * t699 + F::new(1.0);
    let t702 = F::new(1.0) / t701;
    let t703 = t697 * t702;
    let t704 = t703 * t89;
    let t707 = t132 * t61;
    (t698, t699, t701, t702, t703, t704, t707)
}
