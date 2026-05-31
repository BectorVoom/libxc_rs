//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 230/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk230<F: Float>(t265: F, t260: F, t350: F, t405: F, t624: F, t629: F) -> (F, F, F, F) {
    let t659 = t265 * t265;
    let t660 = F::cast_from(1.0_f64) / t659;
    let t661 = t260 * t660;
    let t666 = -F::cast_from(1.176575_f64) * t624 - F::cast_from(0.516475_f64) * t350 - F::cast_from(0.2103875_f64) * t629 - F::cast_from(0.104195_f64) * t405;
    (t659, t660, t661, t666)
}
