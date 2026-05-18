//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 654/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk654<F: Float>(t9037: F, t9082: F, t515: F, t2214: F, t8392: F, t2157: F, t379: F, t604: F, t2210: F, t2225: F, t582: F, t597: F) -> (F, F, F, F, F, F, F) {
    let t9083 = t9037 + t9082;
    let t9084 = t515 * t9083;
    let t9090 = t8392 * t2214;
    let t9093 = t604 * t2157 * t379;
    let t9094 = t2210 * t9093;
    let t9097 = t8392 * t2225;
    let t9099 = t582 * t597;
    (t9083, t9084, t9090, t9093, t9094, t9097, t9099)
}
