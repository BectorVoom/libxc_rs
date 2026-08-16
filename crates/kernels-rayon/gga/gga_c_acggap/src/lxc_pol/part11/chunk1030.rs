//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1030/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1030(t1988: f64, t8541: f64, t30811: f64, t4908: f64, t4680: f64, t7493: f64, t8648: f64, t1421: f64, t1992: f64, t30827: f64, t7842: f64, t1165: f64, t4752: f64, t7351: f64, t7575: f64) -> (f64, f64, f64, f64, f64) {
    let t34170 = t1988 * t8541;
    let t34171 = 0.10718504529517434243e-2_f64 * t34170;
    let t34172 = t30811 * t4908;
    let t34173 = 0.68598428988911579156e-2_f64 * t34172;
    let t34175 = t7493 * t4680 * t8648;
    let t34176 = 0.10718504529517434243e-2_f64 * t34175;
    let t34179 = t30827 * t7842 * t1992 * t1421;
    let t34183 = t7575 * t1165 * t7351 * t4752;
    (t34171, t34173, t34176, t34179, t34183)
}
