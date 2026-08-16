//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2392/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2392(t273: f64, t270: f64, t276: f64, t39484: f64, t9303: f64, t931: f64, t2922: f64, t275: f64, t2925: f64, t41306: f64, t11384: f64, t910: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41382 = f64::powf(t273, -0.25e1_f64);
    let t41401 = 1.0_f64 / t276 / t39484 / t270 / 96.0_f64;
    let t41441 = t9303 * t931;
    let t41497 = t2922 * t2922;
    let t41499 = t275 / t41497;
    let t41501 = t2925 * t2925;
    let t41502 = 1.0_f64 / t41501;
    let t41520 = 0.96141975308641975307e-1_f64 * t41306;
    let t41549 = 0.18467901234567901234e0_f64 * t41306;
    let t41583 = t910 * t11384;
    (t41382, t41401, t41441, t41499, t41502, t41520, t41549, t41583)
}
