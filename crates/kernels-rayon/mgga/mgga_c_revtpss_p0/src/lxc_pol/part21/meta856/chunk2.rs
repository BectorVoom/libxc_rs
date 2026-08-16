//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3249/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3249(t10318: f64, t10327: f64, t10380: f64, t10407: f64, t13334: f64, t13406: f64, t13409: f64, t13414: f64, t1470: f64, t1471: f64, t1486: f64, t2291: f64, t2312: f64, t4182: f64, t4187: f64, t4188: f64, t4191: f64, t606: f64, t607: f64, t641: f64, t72: f64, t85: f64) -> f64 {
    let t60391 = -t13406 * t641 / 4.0_f64 - t4187 * t2291 * t85 / 4.0_f64 - t13409 * t641 / 2.0_f64 - t1470 * t10380 * t85 / 12.0_f64 - t13414 * t641 / 4.0_f64 - t607 * t13334 * t85 / 4.0_f64 - t4182 * t2312 / 4.0_f64 - t4188 * t2312 / 4.0_f64 - t4191 * t2312 / 4.0_f64 - t1471 * t10407 / 12.0_f64 - t606 * t1486 * t72 * t10318 / 4.0_f64 - t10327 * t1486 * t85 / 12.0_f64;
    t60391
}
