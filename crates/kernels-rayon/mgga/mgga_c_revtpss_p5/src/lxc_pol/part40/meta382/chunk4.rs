//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1379/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1379(t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12610: f64, t16706: f64, t16708: f64, t16711: f64, t16713: f64, t16717: f64, t16722: f64, t16727: f64, t16731: f64, t16735: f64, t16740: f64, t16744: f64, t16748: f64) -> f64 {
    let t16750 = -t12610 + 0.13170370370370370371e-1_f64 * t12297 + 0.32925925925925925927e-2_f64 * t12299 - 0.9877777777777777778e-2_f64 * t12301 - 0.4938888888888888889e-2_f64 * t12303 + 0.65851851851851851853e-2_f64 * t16706 + 0.65851851851851851854e-2_f64 * t16708 - t16711 - t16713 + 0.16462962962962962963e-1_f64 * t16717 - 0.59266666666666666668e-1_f64 * t16722 - 0.19755555555555555556e-1_f64 * t16727 - 0.9877777777777777778e-2_f64 * t16731 + 0.88900000000000000002e-1_f64 * t16735 + 0.59266666666666666668e-1_f64 * t16740 + 0.29633333333333333334e-1_f64 * t16744 + 0.14816666666666666667e-1_f64 * t16748;
    t16750
}
