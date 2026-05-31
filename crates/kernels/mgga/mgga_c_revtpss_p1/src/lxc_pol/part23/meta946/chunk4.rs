//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3120/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3120<F: Float>(t43888: F, t56236: F, t58607: F, t58609: F, t58624: F, t68332: F, t68334: F, t68336: F, t68389: F, t68399: F, t68454: F, t68456: F, t81224: F, t81228: F, t81230: F, t81232: F, t81234: F, t81236: F, t81242: F, t81245: F) -> F {
    let t82111 = F::cast_from(0.11415555555555555555e-1_f64) * t68332 + F::cast_from(0.2283111111111111111e-1_f64) * t68334 + F::cast_from(0.6849333333333333333e-1_f64) * t68336 - t58607 + t58609 + F::cast_from(0.30822e0_f64) * t81224 + F::cast_from(0.17123333333333333333e-1_f64) * t81228 - F::cast_from(0.63419753086419753083e-2_f64) * t81230 + F::cast_from(0.2283111111111111111e-1_f64) * t81232 - F::cast_from(0.34246666666666666667e-1_f64) * t81234 - F::cast_from(0.57077777777777777777e-2_f64) * t81236 + t58624 - F::cast_from(0.5327259259259259259e-1_f64) * t56236 - F::cast_from(0.17123333333333333333e-1_f64) * t68389 + F::cast_from(0.4566222222222222222e-1_f64) * t68399 + F::cast_from(0.57077777777777777775e-1_f64) * t81242 - F::cast_from(0.20547999999999999999e0_f64) * t81245 - F::cast_from(0.17757530864197530864e-1_f64) * t43888 - F::cast_from(0.6849333333333333333e-1_f64) * t68454 - F::cast_from(0.10274e0_f64) * t68456;
    t82111
}
