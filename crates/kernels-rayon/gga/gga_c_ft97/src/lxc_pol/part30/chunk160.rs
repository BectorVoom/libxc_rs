//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 160/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk160(t238: f64, t1127: f64, t695: f64, t1097: f64, t1111: f64, t1115: f64, t224: f64, t678: f64) -> (f64, f64) {
    let t239 = 0.1e-59_f64 < t238;
    let t1128 = t695 * t1127;
    let t1131 = piecewise3(t239, -0.11627450473218896e-1_f64 * t678 * t1097 + 2.0_f64 * t1115 + 0.59273806478425129876e-2_f64 * t238 * t1111 - t224 * t1128, 0.0_f64);
    (t1128, t1131)
}
