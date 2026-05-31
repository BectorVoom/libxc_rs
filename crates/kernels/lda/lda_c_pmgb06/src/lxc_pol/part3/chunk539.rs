//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 539/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk539<F: Float>(t107: F, t2060: F, t247: F, t2781: F, t2786: F, t93: F) -> F {
    let t2789 = F::cast_from(7.0_f64) / F::cast_from(27.0_f64) * t93 * t2781 - F::cast_from(0.06068888888888889_f64) * t2060 + F::cast_from(0.01829167760955153_f64) * t247 - F::cast_from(0.0036147222222222223_f64) * t107 * t2786;
    t2789
}
