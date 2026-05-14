//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 742/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk742<F: Float>(t19233: F, t5261: F, t1209: F, t5231: F, t1208: F, t5284: F, t10364: F, t285: F, t21249: F, t281: F, t21253: F, t287: F, t290: F, t1197: F, t5266: F, t283: F, t5230: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22063 = t19233 * t5261;
    let t22065 = t5231 * t1209;
    let t22067 = t5284 * t1208;
    let t22068 = t10364 * t22067;
    let t22069 = t285 * t22068;
    let t22071 = t281 * t21249;
    let t22073 = t21253 * t287 * t290;
    let t22078 = t5266 * t1197;
    let t22081 = t5230 * t283;
    (t22063, t22065, t22067, t22068, t22069, t22071, t22073, t22078, t22081)
}
