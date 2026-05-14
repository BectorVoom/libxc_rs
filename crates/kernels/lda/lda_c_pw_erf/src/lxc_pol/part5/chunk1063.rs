//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1063/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1063<F: Float>(t568: F, t7470: F, t515: F, t7466: F, t10427: F, t2146: F, t6195: F, t2188: F, t6198: F, t4062: F, t571: F, t7488: F, t13750: F, t22121: F, t22125: F, t22129: F, t22133: F, t22137: F, t22141: F) -> (F, F, F, F, F, F, F) {
    let t22142 = t7470 * t568;
    let t22143 = 8.0 / 15.0 * t22142;
    let t22144 = t7466 * t515;
    let t22145 = 8.0 / 15.0 * t22144;
    let t22146 = 16.0 / 405.0 * t10427;
    let t22148 = 4.0 / 5.0 * t2146 * t6195;
    let t22150 = 4.0 / 5.0 * t6198 * t2188;
    let t22152 = t571 * t4062 * t7488;
    let t22153 = 16.0 / 27.0 * t22152;
    let t22154 = t22121 + t22125 - t22129 + t22133 + t22137 - t22141 - t13750 + t22143 + t22145 + t22146 + t22148 + t22150 - t22153;
    (t22143, t22145, t22146, t22148, t22150, t22153, t22154)
}
