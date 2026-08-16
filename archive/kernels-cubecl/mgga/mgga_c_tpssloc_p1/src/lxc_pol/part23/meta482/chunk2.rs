//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1453/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1453<F: Float>(t78298: F, t78302: F, t78304: F, t78310: F, t78312: F, t78314: F, t78318: F, t78320: F, t78327: F, t78329: F, t78331: F, t78333: F) -> F {
    let t78794 = -t78298 + t78302 - t78304 + t78310 - t78312 - t78314 - t78318 - t78320 + t78327 + t78329 + t78331 + t78333;
    t78794
}
