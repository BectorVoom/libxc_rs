//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 698/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk698(t1852: f64, t20268: f64, t83: f64, t11550: f64, t11578: f64, t16192: f64, t16213: f64, t1901: f64, t20226: f64, t20230: f64, t20233: f64, t20236: f64, t20240: f64, t20244: f64, t20248: f64, t20256: f64, t20260: f64, t20265: f64, t446: f64) -> (f64, f64, f64) {
    let t20269 = t1852 * t20268;
    let t20270 = t83 * t20269;
    let t20273 = 2.0_f64 / 3.0_f64 * t1901 * t20226 - 2.0_f64 / 9.0_f64 * t1901 * t20230 + 2.0_f64 / 3.0_f64 * t1901 * t20233 + 2.0_f64 / 3.0_f64 * t1901 * t20236 - 2.0_f64 / 3.0_f64 * t1901 * t20240 + 4.0_f64 / 9.0_f64 * t446 * t20244 + 2.0_f64 / 3.0_f64 * t446 * t20248 - 4.0_f64 / 9.0_f64 * t11550 - 2.0_f64 / 9.0_f64 * t16192 + 4.0_f64 / 9.0_f64 * t11578 + t16213 / 3.0_f64 - t446 * t20256 / 9.0_f64 - 10.0_f64 / 81.0_f64 * t446 * t20260 - 2.0_f64 * t446 * t20265 + 2.0_f64 * t446 * t20270;
    (t20269, t20270, t20273)
}
