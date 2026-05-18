//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 742/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk742<F: Float>(t1872: F, t464: F, t477: F, t137: F, t132: F, t2108: F, t432: F, t1848: F, t531: F, t1397: F, t802: F, t1887: F, t479: F) -> (F, F, F, F, F, F, F, F) {
    let t4815 = t1872 * t464;
    let t4816 = t4815 * t477;
    let t4817 = t137 * t4816;
    let t4819 = t132 * t4817 / F::new(15.0);
    let t4821 = t432 * t2108 / F::new(15.0);
    let t4823 = t1848 * t531 / F::new(15.0);
    let t4825 = t802 * t1397 / F::new(15.0);
    let t4827 = t1887 * t479 / F::new(15.0);
    (t4815, t4816, t4817, t4819, t4821, t4823, t4825, t4827)
}
