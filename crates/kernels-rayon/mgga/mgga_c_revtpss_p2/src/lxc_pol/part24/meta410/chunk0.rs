//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1352/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1352(t2851: f64, t25273: f64, t268: f64, t271: f64, t11852: f64, t159: f64, t273: f64, t270: f64, t276: f64, t39484: f64, t2922: f64, t275: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41295 = t2851 * t2851;
    let t41296 = 1.0_f64 / t41295;
    let t41306 = t268 * t25273 * t271;
    let t41307 = 0.31310740740740740741e1_f64 * t41306;
    let t41329 = 280.0_f64 / 81.0_f64 * t41306;
    let t41339 = t159 * t11852;
    let t41382 = f64::powf(t273, -0.25e1_f64);
    let t41401 = 1.0_f64 / t276 / t39484 / t270 / 96.0_f64;
    let t41497 = t2922 * t2922;
    let t41499 = t275 / t41497;
    (t41296, t41306, t41307, t41329, t41339, t41382, t41401, t41499)
}
