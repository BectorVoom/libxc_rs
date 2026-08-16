//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 882/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk882(t11129: f64, t1156: f64, t1124: f64, t3331: f64, t1136: f64, t3333: f64, t1137: f64, t11282: f64, t440: f64, t11285: f64, t11135: f64, t11203: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11300 = t11129 * t1156;
    let t11303 = t1124 * t3331;
    let t11306 = t3333 * t1136;
    let t11307 = t11306 * t1137;
    let t11310 = t440 * t11282;
    let t11311 = t11129 * t11285;
    let t11314 = 0.16068111111111111111e1_f64 * t11135;
    let t11317 = 0.46308888888888888888e0_f64 * t11203;
    (t11300, t11303, t11306, t11307, t11310, t11311, t11314, t11317)
}
