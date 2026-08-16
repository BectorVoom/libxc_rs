//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 767/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk767<F: Float>(t2476: F, t7341: F, t7342: F, t845: F, t2441: F, t2559: F, t2270: F, t2643: F, t2642: F, t1891: F, t2269: F, t549: F) -> (F, F, F, F, F, F) {
    let t7344 = t7341 * t7342 * t2476;
    let t7346 = F::cast_from(0.1038945353962551798e3_f64) * t845 * t7344;
    let t7348 = F::cast_from(0.35089340384731224426e1_f64) * t2441 * t2559;
    let t7349 = t2643 * t2270;
    let t7350 = t2642 * t7349;
    let t7354 = t2269 * t1891 * t549;
    (t7344, t7346, t7348, t7349, t7350, t7354)
}
