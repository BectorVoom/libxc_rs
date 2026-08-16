//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1030/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1030(t9428: f64, t8809: f64, t8813: f64, t8815: f64, t9435: f64, t9438: f64, t8822: f64, t8183: f64, t8184: f64, t8187: f64, t8190: f64, t9488: f64) -> (f64, f64) {
    let t42515 = 0.79828278012425390428e-1_f64 * t9428;
    let t42516 = 0.20431007948782962912e-3_f64 * t8809;
    let t42517 = 0.5107751987195740728e-4_f64 * t8813;
    let t42518 = 0.5107751987195740728e-4_f64 * t8815;
    let t42519 = 0.4726e1_f64 * t9435;
    let t42520 = 0.11974241701863808564e0_f64 * t9438;
    let t42521 = 0.5987120850931904282e-1_f64 * t8822;
    let t42522 = -t8183 + t8184 + t8187 + t42515 - t8190 + t42516 + t42517 - t42518 - t42519 - t42520 + t42521;
    let t42527 = 0.39914139006212695214e-1_f64 * t9488;
    (t42522, t42527)
}
