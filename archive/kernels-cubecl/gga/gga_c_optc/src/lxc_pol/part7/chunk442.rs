//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 442/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk442<F: Float>(t1773: F, t1782: F, t720: F, t723: F, t182: F, t722: F, t179: F, t727: F) -> (F, F, F, F, F) {
    let t2211 = F::cast_from(0.25851111111111111111e1_f64) * t1773 + F::cast_from(0.20525e-2_f64) * t1782;
    let t2213 = t720 * t723;
    let t2217 = F::cast_from(1.0_f64) / t722 / t182;
    let t2218 = t179 * t2217;
    let t2219 = t727 * t727;
    (t2211, t2213, t2217, t2218, t2219)
}
