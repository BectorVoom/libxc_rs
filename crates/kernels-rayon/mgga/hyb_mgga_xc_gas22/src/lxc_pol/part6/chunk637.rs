//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 637/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk637(t43: f64, t3: f64, t575: f64, t578: f64, t1888: f64, t1891: f64, t3006: f64, t3011: f64, t3017: f64, t3023: f64, t572: f64) -> (f64, f64, f64) {
    let t45 = 0.135e1_f64 < t43;
    let t3025 = t575 * t578 * t3;
    let t3028 = t1888 + t1891 / 162.0_f64 + t3006 / 162.0_f64 - t572 * t3011 / 81.0_f64 + t572 * t3017 / 27.0_f64 - t3023 * t3025 / 27.0_f64;
    let t3029 = piecewise3(t45, t3028, 0.0_f64);
    (t3025, t3028, t3029)
}
