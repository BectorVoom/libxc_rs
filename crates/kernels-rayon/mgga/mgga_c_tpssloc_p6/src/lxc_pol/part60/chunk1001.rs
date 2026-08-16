//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1001/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1001(t114732: f64, t114734: f64, t114737: f64, t114739: f64, t123576: f64, t123578: f64, t126325: f64, t126328: f64, t126332: f64, t126334: f64, t126337: f64, t126339: f64, t126341: f64) -> f64 {
    let t127916 = t123576 - 0.16149102437656156341e-2_f64 * t126325 + 0.32298204875312312682e-2_f64 * t126328 - t123578 + t114732 - t114734 + 0.67826230238155856632e-1_f64 * t126332 + t114737 + t114739 + 5.0_f64 / 192.0_f64 * t126334 + 0.19378922925187387609e-1_f64 * t126337 - t126339 / 96.0_f64 - t126341 / 192.0_f64;
    t127916
}
