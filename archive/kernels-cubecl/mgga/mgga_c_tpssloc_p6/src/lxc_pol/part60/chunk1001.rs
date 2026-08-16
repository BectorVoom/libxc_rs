//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1001/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1001<F: Float>(t114732: F, t114734: F, t114737: F, t114739: F, t123576: F, t123578: F, t126325: F, t126328: F, t126332: F, t126334: F, t126337: F, t126339: F, t126341: F) -> F {
    let t127916 = t123576 - F::cast_from(0.16149102437656156341e-2_f64) * t126325 + F::cast_from(0.32298204875312312682e-2_f64) * t126328 - t123578 + t114732 - t114734 + F::cast_from(0.67826230238155856632e-1_f64) * t126332 + t114737 + t114739 + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t126334 + F::cast_from(0.19378922925187387609e-1_f64) * t126337 - t126339 / F::cast_from(96.0_f64) - t126341 / F::cast_from(192.0_f64);
    t127916
}
