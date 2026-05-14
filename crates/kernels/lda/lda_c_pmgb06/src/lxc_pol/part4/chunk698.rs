//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 698/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk698<F: Float>(t1928: F, t432: F, t1873: F, t435: F, t132: F, t1517: F, t802: F, t1872: F, t464: F, t477: F, t137: F, t2108: F, t1848: F, t531: F, t1397: F, t1887: F, t479: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4809 = 2.0 / 45.0 * t432 * t1928;
    let t4810 = t435 * t1873;
    let t4812 = 2.0 / 45.0 * t132 * t4810;
    let t4814 = 2.0 / 45.0 * t802 * t1517;
    let t4815 = t1872 * t464;
    let t4816 = t4815 * t477;
    let t4817 = t137 * t4816;
    let t4819 = t132 * t4817 / 15.0;
    let t4821 = t432 * t2108 / 15.0;
    let t4823 = t1848 * t531 / 15.0;
    let t4825 = t802 * t1397 / 15.0;
    let t4827 = t1887 * t479 / 15.0;
    (t4809, t4810, t4812, t4814, t4815, t4816, t4817, t4819, t4821, t4823, t4825, t4827)
}
