//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 830/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk830(t1248: f64, t5225: f64, t2862: f64, t871: f64, t1212: f64, t5299: f64, t319: f64, t4246: f64, t5330: f64, t840: f64, t5393: f64, t15147: f64, t1901: f64, t19318: f64, t19320: f64, t19322: f64, t19343: f64, t19387: f64, t19389: f64, t22178: f64, t22183: f64, t22188: f64, t446: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22194 = t5225 * t1248;
    let t22196 = t2862 * t871 * t22194;
    let t22199 = t1212 * t5299;
    let t22201 = t2862 * t319 * t22199;
    let t22205 = t840 * t4246 * t5330;
    let t22208 = t5299 * t1248;
    let t22210 = t840 * t871 * t22208;
    let t22212 = t1212 * t5393;
    let t22214 = t840 * t871 * t22212;
    let t22216 = -2.0_f64 / 3.0_f64 * t19318 + 2.0_f64 / 27.0_f64 * t19320 + t19322 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t1901 * t22178 + 2.0_f64 / 9.0_f64 * t19343 - 2.0_f64 / 3.0_f64 * t1901 * t22183 + 2.0_f64 / 9.0_f64 * t1901 * t22188 - 4.0_f64 / 9.0_f64 * t15147 - 2.0_f64 / 9.0_f64 * t19387 + 2.0_f64 / 3.0_f64 * t19389 - 2.0_f64 * t446 * t22196 + 2.0_f64 * t446 * t22201 + 2.0_f64 * t446 * t22205 + t446 * t22210 + t446 * t22214;
    (t22194, t22196, t22199, t22201, t22205, t22208, t22210, t22212, t22214, t22216)
}
