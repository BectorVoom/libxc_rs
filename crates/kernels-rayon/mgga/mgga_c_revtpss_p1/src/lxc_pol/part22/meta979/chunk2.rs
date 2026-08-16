//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3292/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3292(t10703: f64, t2674: f64, t62403: f64, t10698: f64, t10943: f64, t14586: f64, t14791: f64, t14802: f64, t18444: f64, t23160: f64, t2394: f64, t2745: f64, t4362: f64, t4364: f64, t50511: f64, t50649: f64, t51168: f64, t51170: f64, t5962: f64, t6035: f64, t62385: f64, t62392: f64, t62399: f64, t62401: f64, t825: f64, t827: f64, t828: f64, t851: f64) -> f64 {
    let t62405 = t2674 * t10703 * t62403;
    let t62425 = -0.21437009059034868486e-3_f64 * t825 * t827 * t828 * t62385 - 0.25410001404642664112e-4_f64 * t62392 - 0.25724410870841842183e-1_f64 * t851 * t10698 * t828 * t5962 * t2394 - 0.56688979511669985553e-2_f64 * t62399 + 0.11337795902333997111e-1_f64 * t62401 + 0.50820002809285328225e-3_f64 * t62405 - 0.34299214494455789578e-2_f64 * t4362 * t14791 * t23160 * t14802 - 0.68598428988911579156e-2_f64 * t4362 * t14791 * t14586 * t50649 + 0.17149607247227894789e-2_f64 * t2745 * t14791 * t50511 * t6035 - 0.4065600224742826258e-3_f64 * t51168 + 0.42874018118069736972e-3_f64 * t4362 * t4364 * t18444 * t10943 + 0.57800528129545867621e-2_f64 * t51170;
    t62425
}
