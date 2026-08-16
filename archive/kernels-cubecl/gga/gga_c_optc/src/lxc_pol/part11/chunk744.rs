//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 744/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk744<F: Float>(t1994: F, t3386: F, t3314: F, t622: F, t3313: F, t176: F, t729: F, t3315: F, t1319: F, t6680: F, t188: F, t1990: F) -> (F, F, F, F, F, F) {
    let t9404 = t3386 * t1994;
    let t9411 = t3314 * t622;
    let t9412 = t3313 * t9411;
    let t9415 = t176 * t729;
    let t9416 = t9415 * t3315;
    let t9430 = t6680 * t1319;
    let t9431 = t188 * t9430;
    let t9477 = t3386 * t1990;
    (t9404, t9412, t9416, t9430, t9431, t9477)
}
