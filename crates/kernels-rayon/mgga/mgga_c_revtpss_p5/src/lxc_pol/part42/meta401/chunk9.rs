//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1373/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1373(t1803: f64, t5326: f64, t12297: f64, t12610: f64, t16706: f64, t16708: f64, t16711: f64, t16713: f64, t20283: f64, t20285: f64, t20287: f64, t20290: f64, t20295: f64, t20300: f64, t20304: f64, t20308: f64, t20312: f64, t20315: f64, t20320: f64) -> (f64, f64) {
    let t21063 = t5326 * t1803;
    let t21082 = -t12610 + 0.65851851851851851853e-2_f64 * t12297 + 0.13170370370370370371e-1_f64 * t16706 + 0.65851851851851851853e-2_f64 * t16708 - t16711 - t16713 + 0.32925925925925925927e-2_f64 * t20283 + 0.16462962962962962963e-1_f64 * t20295 - 0.59266666666666666668e-1_f64 * t20300 - 0.19755555555555555556e-1_f64 * t20304 - 0.9877777777777777778e-2_f64 * t20285 + 0.88900000000000000002e-1_f64 * t20308 + 0.59266666666666666668e-1_f64 * t20312 - 0.4938888888888888889e-2_f64 * t20287 - 0.9877777777777777778e-2_f64 * t20315 + 0.29633333333333333334e-1_f64 * t20320 + 0.14816666666666666667e-1_f64 * t20290;
    (t21063, t21082)
}
