//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 458/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk458(t238: f64, t218: f64, t7203: f64, t665: f64, t7205: f64, t1408: f64, t1412: f64, t1420: f64, t6815: f64, t7448: f64, t7453: f64, t7456: f64, t7458: f64, t7466: f64, t7471: f64, t7477: f64) -> (f64, f64, f64, f64) {
    let t239 = 0.1e-59_f64 < t238;
    let t7478 = t7203 * t218;
    let t7479 = t7205 * t665;
    let t7480 = t7478 * t7479;
    let t7484 = piecewise3(t239, 2.0_f64 * t7448 - 0.88910709717637694816e-2_f64 * t1412 * t1408 - 0.76612330055555555556e-1_f64 * t7453 * t1420 + 0.22227677429409423704e-2_f64 * t7456 * t7458 + 0.19762785756235085044e-4_f64 * t238 * t7466 + 0.34058283191806748844e-3_f64 * t6815 * t7471 - 0.22227677429409423704e-2_f64 * t238 * t7458 + 0.58694491165413811142e-2_f64 * t7477 * t7480, 0.0_f64);
    (t7478, t7479, t7480, t7484)
}
