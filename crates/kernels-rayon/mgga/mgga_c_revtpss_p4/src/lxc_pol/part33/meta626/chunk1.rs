//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2069/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2069(t99029: f64, t25266: f64, t4426: f64, t1561: f64, t93048: f64, t14741: f64, t1945: f64, t807: f64, t10886: f64, t4416: f64, t7028: f64, t1549: f64, t92968: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99030 = 0.28582678745379824648e-4_f64 * t99029;
    let t99033 = t25266 * t4426;
    let t99034 = 0.40015750243531754508e-2_f64 * t99033;
    let t99035 = t93048 * t1561;
    let t99041 = t807 * t1945 * t14741;
    let t99042 = 0.11433071498151929859e-3_f64 * t99041;
    let t99044 = t10886 * t7028 * t4416;
    let t99050 = t92968 * t1549;
    (t99030, t99034, t99035, t99042, t99044, t99050)
}
