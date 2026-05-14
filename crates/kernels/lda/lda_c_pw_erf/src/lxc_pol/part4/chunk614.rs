//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 614/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk614<F: Float>(t3117: F, t335: F, t913: F, t935: F, t333: F, t905: F, t334: F, t904: F, t317: F, t902: F, t13: F, t30: F, t906: F, t27: F, t907: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3118 = 48.24547296645331 * t3117;
    let t3120 = t913 * t335 * t935;
    let t3121 = 6.0 * t3120;
    let t3122 = t905 * t333;
    let t3123 = t3122 * t334;
    let t3124 = t904 * t3123;
    let t3125 = 6.0 * t3124;
    let t3127 = 1.0 / t902 / t317;
    let t3128 = t13 * t3127;
    let t3130 = 1.0 / t906 / t30;
    let t3131 = t3122 * t3130;
    let t3132 = t3128 * t3131;
    let t3133 = 517.2501470570617 * t3132;
    let t3135 = 1.0 / t902 / t27;
    let t3136 = t13 * t3135;
    let t3137 = t3122 * t907;
    let t3138 = t3136 * t3137;
    (t3118, t3120, t3121, t3123, t3124, t3125, t3127, t3128, t3130, t3131, t3132, t3133, t3135, t3136, t3137, t3138)
}
