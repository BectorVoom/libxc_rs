//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1194/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1194(t5309: f64, t72231: f64, t1248: f64, t84519: f64, t15195: f64, t15290: f64, t15299: f64, t1901: f64, t19571: f64, t22376: f64, t2881: f64, t296: f64, t446: f64, t4973: f64, t71522: f64, t71532: f64, t71534: f64, t71589: f64, t84080: f64, t84087: f64, t89805: f64, t89809: f64) -> (f64, f64, f64) {
    let t90632 = t72231 * t5309;
    let t90652 = t84519 * t1248;
    let t90664 = 2.0_f64 / 3.0_f64 * t1901 * t2881 * t19571 * t4973 - 16.0_f64 / 9.0_f64 * t71522 - 8.0_f64 / 3.0_f64 * t1901 * t15299 * t89805 + 16.0_f64 / 9.0_f64 * t71532 + 16.0_f64 / 9.0_f64 * t71534 + 4.0_f64 / 3.0_f64 * t84080 - 4.0_f64 / 3.0_f64 * t446 * t296 * t90652 + 4.0_f64 / 3.0_f64 * t84087 + 8.0_f64 / 9.0_f64 * t1901 * t15290 * t89809 - 16.0_f64 / 9.0_f64 * t71589 - 8.0_f64 / 3.0_f64 * t1901 * t15195 * t22376;
    (t90632, t90652, t90664)
}
