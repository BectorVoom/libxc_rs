//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 387/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk387<F: Float>(t1702: F, t554: F, t1701: F, t137: F, t548: F, t135: F) -> (F, F, F, F, F) {
    let t2044 = t1702 * t554;
    let t2045 = t1701 * t2044;
    let t2057 = F::cast_from(1.0_f64) / t548 / t137;
    let t2058 = t135 * t2057;
    let t2059 = t554 * t554;
    (t2044, t2045, t2057, t2058, t2059)
}
