//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 996/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk996(t15168: f64, t15170: f64, t19375: f64, t19380: f64, t19384: f64, t19387: f64, t19389: f64, t19392: f64, t19396: f64, t19401: f64, t19406: f64, t19411: f64, t19415: f64, t19420: f64, t19425: f64, t446: f64) -> f64 {
    let t19428 = t446 * t19375 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t19380 + 4.0_f64 / 3.0_f64 * t446 * t19384 - 2.0_f64 / 27.0_f64 * t19387 - t15168 - t15170 + 2.0_f64 / 9.0_f64 * t19389 - t446 * t19392 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t19396 - 2.0_f64 * t446 * t19401 - 2.0_f64 / 3.0_f64 * t446 * t19406 + 4.0_f64 / 3.0_f64 * t446 * t19411 + 2.0_f64 / 3.0_f64 * t446 * t19415 + 2.0_f64 / 3.0_f64 * t446 * t19420 - 2.0_f64 / 3.0_f64 * t446 * t19425;
    t19428
}
