//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 551/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk551(t40: f64, t52: f64, t2427: f64, t708: f64, t607: f64, t751: f64, t707: f64, t195: f64, t2244: f64, t2250: f64, t73: f64, t197: f64, t76: f64, t157: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t2429 = 8.0_f64 * t2427 * t708;
    let t2430 = t751 * t607;
    let t2431 = t707 * t2430;
    let t2432 = 8.0_f64 * t2431;
    let t2433 = 1.0_f64 / t195;
    let t2439 = piecewise3(t146, 0.0_f64, 4.0_f64 / 9.0_f64 * t2433 * t2244 + 4.0_f64 / 3.0_f64 * t73 * t2250);
    let t2440 = 1.0_f64 / t197;
    let t2446 = piecewise3(t150, 0.0_f64, 4.0_f64 / 9.0_f64 * t2440 * t2244 - 4.0_f64 / 3.0_f64 * t76 * t2250);
    let t2447 = t2439 + t2446;
    let t2448 = t2447 * t157;
    (t2429, t2430, t2431, t2432, t2433, t2440, t2447, t2448)
}
