//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 790/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk790(t31: f64, t9258: f64, t65: f64, t2251: f64, t628: f64, t2283: f64, t608: f64, t36: f64, t366: f64, t41: f64, t42: f64, t2244: f64, t607: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9259 = t31 * t9258;
    let t9260 = t9259 * t65;
    let t9263 = t2251 * t628;
    let t9268 = t608 * t2283;
    let t9276 = 1.0_f64 / t36 / t366;
    let t9277 = sigma0 * t9276;
    let t9287 = 1.0_f64 / t42 / t41;
    let t9288 = t2244 * t607;
    (t9259, t9260, t9263, t9268, t9277, t9287, t9288)
}
