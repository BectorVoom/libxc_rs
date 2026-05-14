//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 579/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk579<F: Float>(t7782: F, t7820: F, t8192: F, t8195: F, t7771: F, t8189: F, t7754: F, t7786: F, t7804: F, t8186: F, t8338: F, t8348: F, t8352: F, t8445: F, t103: F, t82: F) -> (F, F) {
    let t8446 = 2.0 / 27.0 * t7782;
    let t8449 = 2.0 / 9.0 * t7820;
    let t8451 = 4.0 / 9.0 * t8192;
    let t8452 = t8195 / 3.0;
    let t8454 = 2.0 / 3.0 * t7771;
    let t8455 = 28.0 / 81.0 * t8189;
    let t8459 = t8446 - 2.0 / 3.0 * t7786 + 4.0 / 9.0 * t7804 - t8449 - t8186 / 3.0 - t8451 + t8452 - 2.0 * t7754 - t8454 - t8455 + t8338 / 6.0 + t8348 / 8.0 - t8352 / 4.0;
    let t8460 = t8445 + t8459;
    let t8462 = t82 * t8460 * t103;
    (t8460, t8462)
}
