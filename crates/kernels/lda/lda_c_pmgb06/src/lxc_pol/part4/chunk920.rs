//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 920/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk920<F: Float>(t1486: F, t947: F, t1478: F, t1830: F, t508: F, t1482: F, t132: F, t2851: F, t478: F, t3055: F, t432: F, t1396: F, t1547: F, t1540: F, t1592: F, t1595: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9530 = t947 * t1486;
    let t9532 = t947 * t1478;
    let t9552 = t1830 * t508;
    let t9577 = t947 * t1482;
    let t9596 = t132 * t2851 * t478;
    let t9598 = t432 * t3055;
    let t9601 = t132 * t1547 * t1396;
    let t9610 = t1540 * t1592;
    let t9619 = t132 * t1547 * t1595;
    (t9530, t9532, t9552, t9577, t9596, t9598, t9601, t9610, t9619)
}
