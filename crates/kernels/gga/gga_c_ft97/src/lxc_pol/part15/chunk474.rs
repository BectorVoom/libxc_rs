//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 474/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk474<F: Float>(t2258: F, t2259: F, t4417: F, t1073: F, t2266: F, t925: F, t2271: F, t72: F, t4431: F, t632: F) -> (F, F, F, F, F) {
    let t4857 = t2258 * t2259 * t4417;
    let t4861 = t2266 * t925 * t1073;
    let t4865 = t72 * t2271 * t4417;
    let t4869 = t72 * t632 * t4431;
    let t4872 = t1073 * t1073;
    (t4857, t4861, t4865, t4869, t4872)
}
