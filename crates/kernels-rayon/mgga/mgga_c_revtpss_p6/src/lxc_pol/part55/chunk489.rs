//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 489/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk489(t1263: f64, t159: f64, t635: f64, t2304: f64, t1126: f64, t1130: f64, t1129: f64, t418: f64, t408: f64, t406: f64, t409: f64, t3356: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3360 = t159 * t1263;
    let t3361 = t635 * t635;
    let t3362 = 1.0_f64 / t3361;
    let t3367 = 1.0_f64 / t2304;
    let t3379 = t1126 * t1130;
    let t3382 = t1129 * t418;
    let t3383 = 1.0_f64 / t3382;
    let t3384 = t408 * t3383;
    let t3390 = 1.0_f64 / t409 / t406;
    let t3394 = 4.0_f64 / 9.0_f64 * t3356;
    (t3360, t3362, t3367, t3379, t3384, t3390, t3394)
}
