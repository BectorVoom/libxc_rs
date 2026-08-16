//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 224/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk224<F: Float>(t110: F, t269: F, t282: F, t30: F, t619: F, t636: F, t661: F, t668: F, t676: F, t683: F) -> F {
    let t686 = F::cast_from(0.0005323764196666666_f64) * t30 * t110 * t269 + F::cast_from(1.0_f64) * t661 * t668 - t619 - t636 + F::cast_from(0.00018311447306006544_f64) * t30 * t110 * t282 + F::cast_from(0.5848223622634646_f64) * t676 * t683;
    t686
}
