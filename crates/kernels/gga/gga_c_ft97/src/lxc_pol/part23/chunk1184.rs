//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1184/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1184<F: Float>(t309: F, t6273: F, t7640: F, t29219: F, t8392: F, t29223: F, t29226: F, t29229: F, t7107: F, t8232: F, t1466: F, t2399: F, t7027: F, t25485: F, t6963: F, t25488: F) -> (F, F, F, F, F, F, F, F, F) {
    let t114907 = t7640 * t309 * t6273;
    let t114935 = 4.0 / 27.0 * t8392 * t29219;
    let t114938 = 2.0 / 27.0 * t8392 * t29223;
    let t114940 = 4.0 / 27.0 * t8392 * t29226;
    let t114942 = 4.0 / 81.0 * t8392 * t29229;
    let t114979 = t8232 * t7107;
    let t115003 = t1466 * t2399 * t7027;
    let t115016 = t6963 * t25485;
    let t115024 = t6963 * t25488 / 9.0;
    (t114907, t114935, t114938, t114940, t114942, t114979, t115003, t115016, t115024)
}
