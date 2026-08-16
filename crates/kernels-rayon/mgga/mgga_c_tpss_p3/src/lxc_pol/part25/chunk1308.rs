//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1308/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1308(t21060: f64, t5570: f64, t13719: f64, t18454: f64, t13715: f64, t13736: f64, t19476: f64, t13707: f64, t65607: f64, t13711: f64, t13741: f64, t13745: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t69458 = t21060 * t5570;
    let t69489 = t18454 * t13719;
    let t69491 = t18454 * t13715;
    let t69493 = t19476 * t13736;
    let t69495 = t65607 * t13707;
    let t69497 = t19476 * t13711;
    let t69499 = t18454 * t13741;
    let t69501 = t18454 * t13745;
    (t69458, t69489, t69491, t69493, t69495, t69497, t69499, t69501)
}
