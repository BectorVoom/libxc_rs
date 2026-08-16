//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 285/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk285<F: Float>(t698: F, t971: F, t27: F, t653: F, t693: F, t278: F, t674: F) -> (F, F, F, F, F) {
    let t972 = t971 * t698;
    let t974 = t653 * t27;
    let t975 = t974 * t693;
    let t977 = t674 * t278;
    let t978 = F::cast_from(1.0_f64) / t977;
    (t972, t974, t975, t977, t978)
}
