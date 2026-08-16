//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 909/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk909(t107: f64, t2585: f64, t2281: f64, t667: f64, t2333: f64, t626: f64, t2359: f64, t655: f64, t93: f64, t94: f64, t101: f64, t102: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9358 = 154.0_f64 / 27.0_f64 * t2585 * t107;
    let t9359 = t2281 * t667;
    let t9361 = t626 * t2333;
    let t9363 = t626 * t2359;
    let t9364 = t655 * t655;
    let t9365 = 1.0_f64 / t9364;
    let t9383 = t94 * t93;
    let t9384 = 1.0_f64 / t9383;
    let t9397 = t102 * t101;
    (t9358, t9359, t9361, t9363, t9364, t9365, t9384, t9397)
}
