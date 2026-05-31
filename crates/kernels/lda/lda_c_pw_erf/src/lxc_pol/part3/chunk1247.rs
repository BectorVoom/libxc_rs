//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1247/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1247<F: Float>(t156: F, t426: F, t5610: F, t14650: F, t5592: F, t1840: F, t474: F, t5599: F, t5603: F, t431: F, t5578: F, t5594: F) -> (F, F, F, F, F, F) {
    let t14807 = t426 * t156 * t5610;
    let t14813 = t5592 * t14650;
    let t14814 = F::cast_from(11.75232_f64) * t14813;
    let t14816 = t426 * t474 * t1840;
    let t14817 = F::cast_from(2.0_f64) * t14816;
    let t14819 = t426 * t156 * t5599;
    let t14822 = t426 * t156 * t5603;
    let t14837 = t431 * t5578 * t5594;
    (t14807, t14814, t14817, t14819, t14822, t14837)
}
