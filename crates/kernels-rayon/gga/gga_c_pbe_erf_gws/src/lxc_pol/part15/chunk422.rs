//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 422/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk422(t102: f64, t120: f64, t1533: f64, t118: f64, t119: f64, t331: f64, t156: f64, t497: f64, t496: f64, t1504: f64, t506: f64, t10: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1536 = 0.2923025e1_f64 * t102 * t120 * t1533;
    let t1540 = t118 * t119 * t331 * t120 / 9.0_f64;
    let t1541 = t156 * t497;
    let t1542 = t496 * t1541;
    let t1544 = t506 * t1504;
    let t1545 = t10 * t1544;
    (t1536, t1540, t1541, t1542, t1544, t1545)
}
