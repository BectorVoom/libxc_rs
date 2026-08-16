//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 732/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk732<F: Float>(t11501: F, t452: F, t488: F, t10967: F, t83: F, t10962: F, t1922: F, t447: F, t925: F, t1871: F, t3266: F, t499: F) -> (F, F, F, F, F) {
    let t11503 = t452 * t488 * t11501;
    let t11506 = t83 * t10967;
    let t11509 = t83 * t10962;
    let t11513 = t447 * t1922 * t925;
    let t11517 = t1871 * t499 * t3266;
    (t11503, t11506, t11509, t11513, t11517)
}
