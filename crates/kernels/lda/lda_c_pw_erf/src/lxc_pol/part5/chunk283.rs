//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 283/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk283<F: Float>(t920: F, t923: F, t925: F, t929: F, t931: F, t933: F) -> F {
    let t935 = -F::cast_from(0.4219833333333333_f64) * t920 + F::cast_from(0.8439666666666666_f64) * t923 + F::cast_from(0.3986222222222222_f64) * t925 + F::cast_from(0.06825833333333334_f64) * t929 + F::cast_from(0.13651666666666668_f64) * t931 + F::cast_from(0.1369277777777778_f64) * t933;
    t935
}
