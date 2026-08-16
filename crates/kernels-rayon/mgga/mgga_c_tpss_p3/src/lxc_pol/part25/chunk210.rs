//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 210/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk210(t662: f64, t664: f64, t668: f64, t673: f64, t146: f64) -> (f64, f64) {
    let t675 = -0.632975e0_f64 * t662 - 0.29896666666666666667e0_f64 * t664 - 0.1023875e0_f64 * t668 - 0.82156666666666666667e-1_f64 * t673;
    let t676 = 1.0_f64 / t146;
    (t675, t676)
}
