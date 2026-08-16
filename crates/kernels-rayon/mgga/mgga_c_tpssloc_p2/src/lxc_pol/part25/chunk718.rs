//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 718/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk718(t9365: f64, t9366: f64, t2331: f64, t666: f64, t2358: f64, t2261: f64, t93: f64, t94: f64, t2342: f64, t659: f64, t2341: f64, t2248: f64, tau0: f64) -> (f64, f64, f64, f64, f64) {
    let t9367 = t9365 * t9366;
    let t9370 = t2331 * t666;
    let t9371 = t9370 * t2358;
    let t9374 = tau0 * t2261;
    let t9383 = t94 * t93;
    let t9384 = 1.0_f64 / t9383;
    let t9385 = t2342 * t659;
    let t9386 = t9384 * t9385;
    let t9389 = t2341 * t659;
    let t9390 = t9389 * t2248;
    (t9367, t9371, t9374, t9386, t9390)
}
