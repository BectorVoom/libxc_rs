//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3292/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3292<F: Float>(t10703: F, t2674: F, t62403: F, t10698: F, t10943: F, t14586: F, t14791: F, t14802: F, t18444: F, t23160: F, t2394: F, t2745: F, t4362: F, t4364: F, t50511: F, t50649: F, t51168: F, t51170: F, t5962: F, t6035: F, t62385: F, t62392: F, t62399: F, t62401: F, t825: F, t827: F, t828: F, t851: F) -> F {
    let t62405 = t2674 * t10703 * t62403;
    let t62425 = -F::cast_from(0.21437009059034868486e-3_f64) * t825 * t827 * t828 * t62385 - F::cast_from(0.25410001404642664112e-4_f64) * t62392 - F::cast_from(0.25724410870841842183e-1_f64) * t851 * t10698 * t828 * t5962 * t2394 - F::cast_from(0.56688979511669985553e-2_f64) * t62399 + F::cast_from(0.11337795902333997111e-1_f64) * t62401 + F::cast_from(0.50820002809285328225e-3_f64) * t62405 - F::cast_from(0.34299214494455789578e-2_f64) * t4362 * t14791 * t23160 * t14802 - F::cast_from(0.68598428988911579156e-2_f64) * t4362 * t14791 * t14586 * t50649 + F::cast_from(0.17149607247227894789e-2_f64) * t2745 * t14791 * t50511 * t6035 - F::cast_from(0.4065600224742826258e-3_f64) * t51168 + F::cast_from(0.42874018118069736972e-3_f64) * t4362 * t4364 * t18444 * t10943 + F::cast_from(0.57800528129545867621e-2_f64) * t51170;
    t62425
}
