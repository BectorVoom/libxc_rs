//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1312/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1312(t27932: f64, t47300: f64, t26009: f64, t9802: f64, t26004: f64, t3961: f64, t7252: f64, t9700: f64, t94456: f64, t94460: f64, t94462: f64, t94464: f64, t94466: f64, t94468: f64, t94472: f64, t94474: f64, t94477: f64, t94479: f64) -> f64 {
    let t94481 = t27932 * t47300;
    let t94483 = t9802 * t26009;
    let t94484 = 0.91476005056713590805e-4_f64 * t94483;
    let t94485 = t26004 * t3961;
    let t94487 = t7252 * t9700;
    let t94489 = -0.12004725073059526352e-1_f64 * t94456 - 0.34013387707001991332e-1_f64 * t94460 - 0.42874018118069736972e-3_f64 * t94462 + 0.25724410870841842184e-1_f64 * t94464 - 0.42874018118069736972e-3_f64 * t94466 - 0.76230004213927992339e-4_f64 * t94468 - t94472 + t94474 - t94477 + 0.60984003371142393869e-4_f64 * t94479 + 3.0_f64 / 16.0_f64 * t94481 + t94484 + 7.0_f64 / 48.0_f64 * t94485 - t94487 / 48.0_f64;
    t94489
}
