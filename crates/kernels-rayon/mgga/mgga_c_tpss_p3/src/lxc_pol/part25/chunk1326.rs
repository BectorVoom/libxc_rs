//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1326/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1326(t4802: f64, t821: f64, t19817: f64, t4806: f64, t64247: f64, t1288: f64, t3724: f64, t580: f64, t14426: f64, t30: f64, t3610: f64, t14076: f64, t63840: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t70240 = t4802 * t821;
    let t70241 = t19817 * t70240;
    let t70243 = t4806 * t821;
    let t70244 = t64247 * t70243;
    let t70255 = t1288 * t3724;
    let t70258 = t580 * t4806;
    let t70261 = t30 * t14426;
    let t70286 = t1288 * t3610;
    let t70290 = t63840 * t14076;
    (t70240, t70241, t70243, t70244, t70255, t70258, t70261, t70286, t70290)
}
