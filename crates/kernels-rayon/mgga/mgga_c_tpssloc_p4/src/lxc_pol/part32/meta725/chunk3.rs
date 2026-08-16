//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2332/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2332(t210: f64, t29584: f64, t27683: f64, t27710: f64, t1198: f64, t27684: f64, t27692: f64, t27711: f64, t6192: f64, t7331: f64, t8040: f64, t86330: f64, t86348: f64, t86350: f64, t95323: f64, t95556: f64, t95587: f64, t95590: f64, t95593: f64, t95617: f64) -> f64 {
    let t104410 = t29584 * t210;
    let t104413 = t27710 * t27683;
    let t104424 = t86348 / 10368.0_f64 - t86350 / 6912.0_f64 + t95587 - t95590 - t95593 - t95617 - t86330 * t6192 / 1152.0_f64 - 11.0_f64 / 324.0_f64 * t104410 * t1198 + 0.16149102437656156342e-2_f64 * t104413 * t7331 - 0.16149102437656156342e-2_f64 * t27711 * t27692 + 0.16149102437656156342e-2_f64 * t95323 * t8040 - 0.20186378047070195428e-3_f64 * t95556 * t8040 - 0.20186378047070195428e-3_f64 * t27684 * t27692;
    t104424
}
