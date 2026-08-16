//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 747/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk747<F: Float>(t115: F, t5: F, t7192: F, t363: F, t2343: F, t992: F, t355: F, t2529: F, t2534: F, t836: F, t845: F, t529: F, t6: F) -> (F, F, F, F, F, F, F) {
    let t7194 = t7192 * t115 * t5;
    let t7195 = t7194 * t363;
    let t7198 = t2343 * t992;
    let t7199 = t355 * t7198;
    let t7202 = t2529 * t836 * t2534;
    let t7204 = F::cast_from(0.35089340384731224426e1_f64) * t845 * t7202;
    let t7205 = t6 * t529;
    (t7194, t7195, t7198, t7199, t7202, t7204, t7205)
}
