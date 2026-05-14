//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 853/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk853<F: Float>(t128: F, t4864: F, t11202: F, t8291: F, t3640: F, t518: F, t3650: F, t4015: F, t1416: F, t674: F) -> (F, F, F, F, F, F) {
    let t11227 = t4864 * t128;
    let t11228 = t11202 * t11227;
    let t11229 = t11228 * t8291;
    let t11231 = t518 * t3640;
    let t11234 = t3650 * t4015;
    let t11235 = t1416 * t674;
    (t11227, t11228, t11229, t11231, t11234, t11235)
}
