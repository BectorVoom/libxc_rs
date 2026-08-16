//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1130/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1130(t140: f64, t5368: f64, t1222: f64, t3624: f64, t5436: f64, t12772: f64, t5401: f64, t3625: f64, t1214: f64, t1250: f64, t3698: f64, t5047: f64) -> (f64, f64, f64, f64, f64) {
    let t17445 = t140 * t5368;
    let t17447 = t1222 * t17445 / 432.0_f64;
    let t17448 = t5436 * t3624;
    let t17451 = t12772 * t5401;
    let t17453 = 0.19055119163586549765e-3_f64 * t3625 * t17451;
    let t17459 = t1250 * t1214;
    let t17471 = t140 * t3698;
    let t17472 = t17471 * t5047;
    (t17447, t17448, t17453, t17459, t17472)
}
