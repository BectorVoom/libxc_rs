//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 871/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk871<F: Float>(t3903: F, t643: F, t3934: F, t638: F, t1058: F, t696: F, t965: F, t3724: F, t3758: F, t963: F, t3729: F, t971: F) -> (F, F, F, F, F) {
    let t8749 = t643 * t3903;
    let t8751 = t638 * t3934;
    let t8755 = F::cast_from(21.053605041484726_f64) * t696 * t965 * t1058;
    let t8759 = F::cast_from(69.26343642272586_f64) * t696 * t963 * t3758 * t3724;
    let t8760 = t971 * t3729;
    (t8749, t8751, t8755, t8759, t8760)
}
