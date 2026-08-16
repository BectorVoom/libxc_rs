//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 802/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk802(t40: f64, t52: f64, t2250: f64, t73: f64, t9258: f64, t9288: f64, t9427: f64, t9430: f64, t197: f64, t2440: f64, t607: f64, t76: f64, t145: f64, zeta_threshold: f64) -> (f64, f64) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t9436 = piecewise3(t146, 0.0_f64, -8.0_f64 / 27.0_f64 * t9427 * t9288 + 4.0_f64 / 3.0_f64 * t9430 * t2250 + 4.0_f64 / 3.0_f64 * t73 * t9258);
    let t9438 = 1.0_f64 / t197 / t52;
    let t9441 = t2440 * t607;
    let t9447 = piecewise3(t150, 0.0_f64, 8.0_f64 / 27.0_f64 * t9438 * t9288 + 4.0_f64 / 3.0_f64 * t9441 * t2250 - 4.0_f64 / 3.0_f64 * t76 * t9258);
    let t9448 = t9436 + t9447;
    let t9449 = t145 * t9448;
    (t9448, t9449)
}
