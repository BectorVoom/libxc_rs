//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1356/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1356<F: Float>(t13840: F, t13842: F, t13844: F, t13846: F, t13848: F, t13850: F, t13883: F, t13885: F, t13887: F, t17819: F, t17822: F, t17824: F, t17828: F, t17829: F, t17830: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17831 = F::cast_from(32.0_f64) / F::cast_from(135.0_f64) * t13840;
    let t17832 = F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t13842;
    let t17833 = F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t13844;
    let t17834 = F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t13846;
    let t17835 = F::cast_from(32.0_f64) / F::cast_from(243.0_f64) * t13848;
    let t17836 = F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t13850;
    let t17837 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t13883;
    let t17838 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t13885;
    let t17839 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t13887;
    let t17840 = -t17819 + t17822 - t17824 - t17828 - t17829 - t17830 + t17831 + t17832 + t17833 + t17834 + t17835 - t17836 - t17837 - t17838 - t17839;
    (t17831, t17832, t17833, t17834, t17835, t17836, t17837, t17838, t17839, t17840)
}
