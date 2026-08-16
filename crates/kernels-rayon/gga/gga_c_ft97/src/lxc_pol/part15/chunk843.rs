//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 843/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk843(t1212: f64, t4635: f64, t2875: f64, t2874: f64, t1248: f64, t2882: f64, t2881: f64, t15318: f64, t1901: f64, t19635: f64, t22261: f64, t22348: f64, t22352: f64, t22357: f64, t22361: f64, t22364: f64, t22369: f64, t22373: f64, t22377: f64, t22380: f64, t22383: f64, t446: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22386 = t4635 * t1212;
    let t22387 = t2875 * t22386;
    let t22388 = t2874 * t22387;
    let t22391 = t4635 * t1248;
    let t22392 = t2882 * t22391;
    let t22393 = t2881 * t22392;
    let t22396 = 2.0_f64 / 3.0_f64 * t446 * t22261 - t446 * t22348 / 3.0_f64 - 2.0_f64 * t446 * t22352 - t19635 / 3.0_f64 + 2.0_f64 * t446 * t22357 - t446 * t22361 + 2.0_f64 * t446 * t22364 - 4.0_f64 / 27.0_f64 * t15318 + 2.0_f64 / 9.0_f64 * t1901 * t22369 - 2.0_f64 / 9.0_f64 * t1901 * t22373 - 2.0_f64 / 3.0_f64 * t1901 * t22377 + 2.0_f64 / 3.0_f64 * t1901 * t22380 + 2.0_f64 / 3.0_f64 * t1901 * t22383 + t1901 * t22388 / 3.0_f64 + t1901 * t22393 / 3.0_f64;
    (t22386, t22387, t22388, t22391, t22392, t22393, t22396)
}
