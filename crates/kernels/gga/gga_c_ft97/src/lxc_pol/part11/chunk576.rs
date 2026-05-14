//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 576/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk576<F: Float>(t1647: F, t447: F, t499: F, t103: F, t1755: F, t379: F, t1902: F, t375: F, t443: F, t444: F) -> (F, F, F, F, F) {
    let t8383 = t447 * t499 * t1647;
    let t8386 = t103 * t1755;
    let t8387 = t8386 * t379;
    let t8388 = t1902 * t8387;
    let t8392 = t443 * t444 * t375;
    (t8383, t8386, t8387, t8388, t8392)
}
