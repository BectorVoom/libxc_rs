//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 251/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk251(t109: f64, t656: f64, t666: f64, t64: f64, t654: f64) -> (f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t667 = t656 * t666;
    let t671 = piecewise3(t110, 0.0_f64, -t654 - t64 * t667 / 8.0_f64);
    (t667, t671)
}
