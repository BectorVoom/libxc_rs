//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 879/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk879<F: Float>(t13: F, t3130: F, t8185: F, t902: F, t911: F, t1030: F, t2983: F, t400: F, t8171: F, t3148: F, t333: F, t904: F, t907: F) -> (F, F, F) {
    let t8244 = F::cast_from(6207.00176468474_f64) * t13 / t902 / t911 * t8185 * t3130;
    let t8248 = F::cast_from(623.3672123775311_f64) * t400 * t2983 * t8171 * t1030;
    let t8260 = F::cast_from(64.32729728860441_f64) * t904 * t3148 * t907 * t333;
    (t8244, t8248, t8260)
}
