//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 131/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk131(t275: f64, t291: f64, t148: f64, t154: f64, t157: f64, zeta_threshold: f64) -> (f64, f64) {
    let t293 = 0.621814e-1_f64 * t275 * t291;
    let t294 = 2.0_f64 <= zeta_threshold;
    let t296 = piecewise3(t294, t148, 2.0_f64 * t154);
    let t297 = 0.0_f64 <= zeta_threshold;
    let t298 = piecewise3(t297, t148, 0.0_f64);
    let t300 = (t296 + t298 - 2.0_f64) * t157;
    (t293, t300)
}
