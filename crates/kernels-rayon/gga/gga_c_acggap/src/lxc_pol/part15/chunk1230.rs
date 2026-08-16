//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1230/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1230(t34843: f64, t34844: f64, t34846: f64, t34847: f64, t34848: f64, t34849: f64, t37267: f64, t37268: f64, t37276: f64, t37277: f64, t37278: f64, t39525: f64, t39527: f64, t39534: f64, t39537: f64, t39540: f64, t39545: f64, t39547: f64) -> f64 {
    let t41693 = -t39525 / 8.0_f64 - t37267 + t37268 - 7.0_f64 / 144.0_f64 * t39527 + t34843 + 0.68598428988911579156e-2_f64 * t34844 + t34846 - t34847 + t34848 - 0.2264262644851498949e-1_f64 * t34849 - 0.42874018118069736972e-3_f64 * t39534 - 0.42874018118069736972e-3_f64 * t39537 - 0.42874018118069736972e-3_f64 * t39540 + t37276 - t37277 + t37278 - 0.28582678745379824648e-3_f64 * t39545 - 0.21437009059034868486e-3_f64 * t39547;
    t41693
}
