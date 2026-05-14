//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 400/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk400<F: Float>(t2200: F, t831: F, t1454: F, t268: F, t282: F, t285: F) -> (F, F, F, F) {
    let t2201 = t2200 * t831;
    let t2204 = t1454 * t268;
    let t2205 = t285 * t282;
    let t2206 = 1.0 / t2205;
    (t2201, t2204, t2205, t2206)
}
