//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 791/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk791(t16279: f64, t83: f64, t11846: f64, t11849: f64, t16230: f64, t16234: f64, t16238: f64, t16243: f64, t16248: f64, t16252: f64, t16255: f64, t16258: f64, t16263: f64, t16268: f64, t16272: f64, t16276: f64, t1901: f64, t446: f64) -> f64 {
    let t16280 = t83 * t16279;
    let t16284 = -2.0_f64 / 9.0_f64 * t1901 * t16230 + 2.0_f64 / 27.0_f64 * t1901 * t16234 + 2.0_f64 / 27.0_f64 * t1901 * t16238 + t1901 * t16243 / 9.0_f64 - t446 * t16248 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t16252 - 2.0_f64 / 27.0_f64 * t16255 + 2.0_f64 / 3.0_f64 * t446 * t16258 + 2.0_f64 / 3.0_f64 * t446 * t16263 + t446 * t16268 / 3.0_f64 + 4.0_f64 / 3.0_f64 * t446 * t16272 + 2.0_f64 / 3.0_f64 * t446 * t16276 + 4.0_f64 / 3.0_f64 * t446 * t16280 - 8.0_f64 / 27.0_f64 * t11846 + t11849;
    t16284
}
