//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 783/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk783<F: Float>(t7342: F, t7501: F, t7504: F, t845: F, t2441: F, t2468: F, t217: F, t226: F, t2383: F, t782: F, t2382: F, t2391: F) -> (F, F, F, F, F, F, F, F) {
    let t7505 = t7501 * t7342 * t7504;
    let t7507 = F::cast_from(0.1025389702100779493e4_f64) * t845 * t7505;
    let t7509 = F::cast_from(0.17544670192365612213e1_f64) * t2441 * t2468;
    let t7512 = F::cast_from(1.0_f64) / t217 / t226 / F::cast_from(4.0_f64);
    let t7513 = t2383 * t782;
    let t7514 = t7512 * t7513;
    let t7516 = t2382 * t782;
    let t7517 = t7516 * t2391;
    (t7505, t7507, t7509, t7512, t7513, t7514, t7516, t7517)
}
