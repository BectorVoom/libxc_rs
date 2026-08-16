//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 716/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk716(t3524: f64, t636: f64, t108: f64, t2: f64, t555: f64, t105: f64, t1325: f64, t1327: f64, t3515: f64, t3519: f64, t631: f64, t637: f64, t97: f64) -> (f64, f64) {
    let t3525 = t3524 * t636;
    let t3528 = t108 * t2;
    let t3529 = t3528 * t555;
    let t3532 = -25.0_f64 / 9.0_f64 * t631 * t1325 + 10.0_f64 / 9.0_f64 * t97 * t3515 + 5.0_f64 / 3.0_f64 * t97 * t3519 - 25.0_f64 / 9.0_f64 * t1327 * t637 + 10.0_f64 / 9.0_f64 * t105 * t3525 - 5.0_f64 / 3.0_f64 * t105 * t3529;
    (t3528, t3532)
}
