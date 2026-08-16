//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 760/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk760(t2683: f64, t375: f64, t89: f64, t793: f64, t9733: f64, t2336: f64, t2675: f64, t10243: f64, t10246: f64, t10251: f64, t10255: f64, t10259: f64, t10265: f64, t10269: f64, t10273: f64) -> (f64, f64, f64, f64) {
    let t10276 = t89 * t375 * t2683;
    let t10279 = t89 * t9733 * t793;
    let t10282 = t89 * t2336 * t2675;
    let t10284 = -t10243 / 9.0_f64 - t10246 / 9.0_f64 - t10251 / 3.0_f64 - t10255 / 3.0_f64 - t10259 / 18.0_f64 - t10265 + t10269 - 5.0_f64 / 81.0_f64 * t10273 - t10276 / 3.0_f64 - 2.0_f64 / 27.0_f64 * t10279 + t10282 / 18.0_f64;
    (t10276, t10279, t10282, t10284)
}
