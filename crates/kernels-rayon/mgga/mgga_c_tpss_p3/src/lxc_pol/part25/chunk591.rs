//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 591/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk591(t1250: f64, t3342: f64, t508: f64, t526: f64, t235: f64, t72: f64, t1254: f64, t219: f64, t1257: f64, t536: f64, t73: f64, t3255: f64, t532: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3343 = t3342 * t1250;
    let t3346 = 1.0_f64 / t526 / t508;
    let t3347 = t235 * t3346;
    let t3348 = t3347 * t72;
    let t3360 = t1254 * t219;
    let t3364 = 1.0_f64 / t1257 / t536;
    let t3365 = t73 * t3364;
    let t3370 = t3255 * t532;
    (t3343, t3346, t3348, t3360, t3364, t3365, t3370)
}
