//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 701/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk701<F: Float>(t309: F, t6700: F, t1950: F, t2006: F, t2127: F, t4762: F, t95: F, t476: F, t132: F, t4280: F, t481: F, t1986: F, t2021: F) -> (F, F, F, F, F, F, F) {
    let t6701 = t6700 * t309;
    let t6702 = t6701 * t1950;
    let t6704 = t2006 * t2127;
    let t6709 = t4762 * t95;
    let t6710 = t476 * t6709;
    let t6711 = F::cast_from(9.813265947244027_f64) * t6710;
    let t6713 = t481 * t4280 * t132;
    let t6714 = F::cast_from(3.0001361899701053_f64) * t6713;
    let t6719 = t1986 * t2021;
    (t6702, t6704, t6710, t6711, t6713, t6714, t6719)
}
