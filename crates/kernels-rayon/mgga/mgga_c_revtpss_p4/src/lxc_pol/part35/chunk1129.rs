//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1129/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1129(t25944: f64, t96255: f64, t7514: f64, t9288: f64, t7289: f64, t94471: f64, t94473: f64, t94476: f64, t94483: f64, t94522: f64, t94525: f64, t94568: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t96257 = 0.68540937416128198417e-2_f64 * t25944 * t96255;
    let t96282 = t7514 * t9288;
    let t96284 = 0.39982213492741449076e-1_f64 * t7289 * t96282;
    let t96321 = 455.0_f64 / 648.0_f64 * t94471;
    let t96322 = 0.51384669507166276316e-2_f64 * t94473;
    let t96323 = 0.3252886739816735289e-3_f64 * t94476;
    let t96326 = 0.18295201011342718161e-3_f64 * t94483;
    let t96341 = 0.15117061203111996147e0_f64 * t94522;
    let t96342 = 0.80328230880474379779e-6_f64 * t94525;
    let t96358 = 0.45178982497454656792e-6_f64 * t94568;
    (t96257, t96282, t96284, t96321, t96322, t96323, t96326, t96341, t96342, t96358)
}
