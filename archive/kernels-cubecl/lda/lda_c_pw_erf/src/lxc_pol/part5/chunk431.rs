//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 431/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk431<F: Float>(t1944: F, t2017: F, t571: F, t558: F, t833: F, t352: F) -> (F, F, F, F) {
    let t2018 = t2017 * t1944;
    let t2020 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t571 * t2018;
    let t2021 = t833 * t558;
    let t2022 = t2021 * t352;
    (t2018, t2020, t2021, t2022)
}
