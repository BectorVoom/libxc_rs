//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 749/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk749<F: Float>(t2007: F, t4804: F, t1319: F, t4693: F, t571: F, t2017: F, t4671: F, t4689: F, t4758: F, t1472: F, t2018: F, t1351: F, t833: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4806 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t4804 * t2007;
    let t4807 = t1319 * t4693;
    let t4809 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t571 * t4807;
    let t4810 = t2017 * t4671;
    let t4812 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t571 * t4810;
    let t4813 = t4758 * t4689;
    let t4815 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t571 * t4813;
    let t4817 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t1472 * t2018;
    let t4818 = t833 * t1351;
    (t4806, t4807, t4809, t4810, t4812, t4813, t4815, t4817, t4818)
}
