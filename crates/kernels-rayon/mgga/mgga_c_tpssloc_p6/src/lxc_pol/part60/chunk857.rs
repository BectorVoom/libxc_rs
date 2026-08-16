//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 857/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk857(t6547: f64, t8557: f64, t2047: f64, t234: f64, t794: f64, t8556: f64, t6562: f64, t814: f64, t8543: f64, t23204: f64, t8547: f64, t225: f64, t8544: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31374 = t6547 * t8557;
    let t31375 = 0.19190897446562641759e-1_f64 * t31374;
    let t31376 = t234 * t2047;
    let t31381 = t794 * t8556;
    let t31382 = t6562 * t31381;
    let t31383 = 0.41123351671205660912e-2_f64 * t31382;
    let t31394 = t814 * t8543;
    let t31405 = t23204 * t8547;
    let t31406 = t6562 * t31405;
    let t31407 = 0.41123351671205660912e-2_f64 * t31406;
    let t31423 = t8544 * t225;
    (t31375, t31376, t31381, t31383, t31394, t31405, t31407, t31423)
}
