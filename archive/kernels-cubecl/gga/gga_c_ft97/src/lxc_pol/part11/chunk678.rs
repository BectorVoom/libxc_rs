//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 678/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk678<F: Float>(t2101: F, t597: F, t2224: F, t160: F, t2075: F, t379: F, t2221: F, t2133: F, t604: F, t609: F, t144: F, t24: F, t7368: F) -> (F, F, F, F, F, F, F, F) {
    let t9419 = t2101 * t597;
    let t9420 = t9419 * t2224;
    let t9424 = t160 * t2075 * t379;
    let t9425 = t2221 * t9424;
    let t9428 = t2133 * t604;
    let t9429 = t9428 * t609;
    let t9430 = t144 * t9429;
    let t9432 = t24 * t7368;
    (t9419, t9420, t9424, t9425, t9428, t9429, t9430, t9432)
}
