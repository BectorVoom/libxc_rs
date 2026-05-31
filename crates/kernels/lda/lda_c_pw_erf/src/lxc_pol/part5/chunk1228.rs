//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1228/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1228<F: Float>(t4062: F, t571: F, t7488: F, t13750: F, t22121: F, t22125: F, t22129: F, t22133: F, t22137: F, t22141: F, t22143: F, t22145: F, t22146: F, t22148: F, t22150: F) -> (F, F) {
    let t22152 = t571 * t4062 * t7488;
    let t22153 = F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t22152;
    let t22154 = t22121 + t22125 - t22129 + t22133 + t22137 - t22141 - t13750 + t22143 + t22145 + t22146 + t22148 + t22150 - t22153;
    (t22153, t22154)
}
