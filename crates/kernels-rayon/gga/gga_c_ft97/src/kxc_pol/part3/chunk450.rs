//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 450/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk450(t110: f64, t3052: f64, t447: f64, t1882: f64, t951: f64, t3216: f64, t3221: f64, t3224: f64, t3227: f64, t3231: f64, t3235: f64, t3240: f64, t3257: f64, t3260: f64, t3263: f64, t3268: f64, t3273: f64, t3277: f64, t3281: f64, t446: f64) -> (f64, f64, f64) {
    let t3283 = t447 * t110 * t3052;
    let t3286 = t1882 * t951;
    let t3288 = t446 * t3216 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t3221 + t3224 / 9.0_f64 - t446 * t3227 / 3.0_f64 - t446 * t3231 / 3.0_f64 - t446 * t3235 / 3.0_f64 - t446 * t3240 / 3.0_f64 - t446 * t3257 / 3.0_f64 + t3260 / 9.0_f64 - t446 * t3263 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t3268 + t446 * t3273 / 3.0_f64 - t446 * t3277 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t3281 * t3283 + t3286 / 27.0_f64;
    (t3283, t3286, t3288)
}
