//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1436/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1436<F: Float>(t19270: F, t193: F, t336: F, t3640: F, t4700: F, t6270: F, t78310: F, t78312: F, t78314: F, t78318: F, t78320: F, t78321: F, t78327: F, t78329: F, t78331: F, t78333: F, t78335: F, t78338: F) -> F {
    let t78342 = -F::cast_from(3.0_f64) * t193 * t336 * t3640 * t78321 + F::cast_from(12.0_f64) * t19270 * t4700 * t6270 + t78310 - t78312 - t78314 - t78318 - t78320 + t78327 + t78329 + t78331 + t78333 + t78335 + t78338;
    t78342
}
