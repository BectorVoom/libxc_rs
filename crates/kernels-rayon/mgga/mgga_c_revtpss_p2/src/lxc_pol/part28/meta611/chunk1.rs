//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2134/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2134(t1931: f64, t2371: f64, t13426: f64, t13544: f64, t1519: f64, t18153: f64, t18163: f64, t1932: f64, t2372: f64, t25805: f64, t27145: f64, t28025: f64, t28030: f64, t4254: f64, t4257: f64, t4293: f64, t6985: f64, t7007: f64, t7746: f64, t98472: f64, t98474: f64, t98477: f64, t98483: f64, t98486: f64, t98489: f64, t98491: f64, t98494: f64, t98499: f64, t98501: f64) -> (f64, f64) {
    let t98507 = t1931 * t2371;
    let t98512 = -4.0_f64 * t13426 * t7007 - 2.0_f64 * t13544 * t6985 - 2.0_f64 * t1519 * t98507 - t18153 * t1932 - 2.0_f64 * t18163 * t7746 - 2.0_f64 * t2372 * t28030 - 4.0_f64 * t25805 * t4293 - 4.0_f64 * t27145 * t4254 - 4.0_f64 * t28025 * t4257 - t98472 - t98474 - t98477 - t98483 - t98486 - t98489 - t98491 - t98494 - t98499 + t98501;
    (t98507, t98512)
}
