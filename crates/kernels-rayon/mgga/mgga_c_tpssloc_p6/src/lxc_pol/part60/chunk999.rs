//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 999/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk999(t1888: f64, t23270: f64, t31332: f64, t5657: f64, t101593: f64, t114815: f64, t121431: f64, t121437: f64, t121444: f64, t121464: f64, t121469: f64, t126363: f64, t126368: f64, t1911: f64, t1912: f64, t2054: f64, t26700: f64, t2718: f64, t28307: f64, t29055: f64, t29056: f64, t33405: f64, t6627: f64, t7087: f64, t7538: f64, t855: f64, t98239: f64, t98975: f64, t99010: f64) -> f64 {
    let t127874 = t1888 * t23270 * t31332 * t5657;
    let t127883 = 0.38381794893125283518e-1_f64 * t121431 - 0.76763589786250567036e-1_f64 * t121437 - 0.16449340668482264365e-1_f64 * t121444 - t114815 - 12.0_f64 * t98975 * t33405 - 2.0_f64 * t26700 * t7538 + 0.16449340668482264365e-1_f64 * t121464 + 2.0_f64 * t855 * t2718 * t29055 * t1911 + 0.76763589786250567036e-1_f64 * t121469 + 0.16449340668482264365e-1_f64 * t127874 - t99010 * t2054 + 4.0_f64 * t7087 * t28307 - t126363 - t6627 * t29056 + t126368 - 2.0_f64 * t98239 * t2054 - t101593 * t1912;
    t127883
}
