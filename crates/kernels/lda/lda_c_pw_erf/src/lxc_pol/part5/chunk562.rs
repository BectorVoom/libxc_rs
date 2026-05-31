//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 562/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk562<F: Float>(t1022: F, t1030: F, t385: F, t907: F, t935: F, t333: F, t904: F, t335: F, t913: F, t905: F, t334: F, t317: F, t902: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3111 = t1022 * t1030;
    let t3112 = t3111 * t385;
    let t3115 = t935 * t907;
    let t3117 = t904 * t3115 * t333;
    let t3118 = F::cast_from(48.24547296645331_f64) * t3117;
    let t3120 = t913 * t335 * t935;
    let t3121 = F::cast_from(6.0_f64) * t3120;
    let t3122 = t905 * t333;
    let t3123 = t3122 * t334;
    let t3124 = t904 * t3123;
    let t3125 = F::cast_from(6.0_f64) * t3124;
    let t3127 = F::cast_from(1.0_f64) / t902 / t317;
    (t3111, t3112, t3115, t3117, t3118, t3120, t3121, t3122, t3123, t3124, t3125, t3127)
}
