//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1863/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1863(t7289: f64, t96282: f64, t26277: f64, t94776: f64, t25950: f64, t26292: f64, t25904: f64, t96245: f64, t94471: f64, t94473: f64, t94476: f64, t94483: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t96284 = 0.39982213492741449076e-1_f64 * t7289 * t96282;
    let t96287 = t94776 * t26277;
    let t96289 = t25950 * t26292;
    let t96298 = t25904 * t96245;
    let t96321 = 455.0_f64 / 648.0_f64 * t94471;
    let t96322 = 0.51384669507166276316e-2_f64 * t94473;
    let t96323 = 0.3252886739816735289e-3_f64 * t94476;
    let t96326 = 0.18295201011342718161e-3_f64 * t94483;
    (t96284, t96287, t96289, t96298, t96321, t96322, t96323, t96326)
}
