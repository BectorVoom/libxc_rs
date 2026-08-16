//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 633/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk633(t312: f64, t5374: f64, t1218: f64, t1253: f64, t301: f64, t317: f64, t5207: f64, t5305: f64, t5310: f64, t5380: f64, t5394: f64, t5422: f64) -> (f64, f64) {
    let t5424 = t5374 * t312;
    let t5429 = -2.0_f64 * t1218 * t1253 - t301 * t5422 - t317 * t5207 - t317 * t5305 + 4.0_f64 * t5310 - 4.0_f64 * t5380 - 2.0_f64 * t5394 + 2.0_f64 * t5424;
    (t5424, t5429)
}
