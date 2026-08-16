//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1530/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1530(t17817: f64, t2988: f64, t17183: f64, t4518: f64, t135: f64, t5844: f64, t973: f64, t10295: f64, t10296: f64, t13642: f64, t13921: f64, t13922: f64, t13923: f64, t17241: f64, t17244: f64, t17247: f64, t17250: f64, t17253: f64, t17256: f64, t17280: f64, t17286: f64, t17288: f64, t17290: f64, t17293: f64) -> (f64, f64, f64, f64) {
    let t17818 = t2988 * t17817;
    let t17821 = t4518 * t17183;
    let t17826 = t135 * t5844;
    let t17827 = t973 * t17826;
    let t17841 = t10295 + 5.0_f64 / 27.0_f64 * t10296 + 10.0_f64 / 27.0_f64 * t13642 - t13921 + t13922 - t13923 - t17286 / 27.0_f64 + 2.0_f64 / 27.0_f64 * t17244 - t17280 / 3.0_f64 + t17241 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t17288 + t17247 - 2.0_f64 / 3.0_f64 * t17250 - t17290 / 9.0_f64 + t17256 / 18.0_f64 - t17253 / 3.0_f64 + t17293 / 6.0_f64;
    (t17818, t17821, t17827, t17841)
}
