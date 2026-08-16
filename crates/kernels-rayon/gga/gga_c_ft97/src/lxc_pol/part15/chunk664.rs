//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 664/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk664(t375: f64, t5300: f64, t89: f64, t5226: f64, t1882: f64, t5214: f64, t5225: f64, t7640: f64, t2336: f64, t5221: f64, t5217: f64, t5209: f64, t9725: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19246 = t89 * t375 * t5300;
    let t19249 = t89 * t375 * t5226;
    let t19278 = t1882 * t5214;
    let t19289 = t7640 * t5225;
    let t19298 = t89 * t2336 * t5221;
    let t19301 = t89 * t2336 * t5217;
    let t19304 = t89 * t9725 * t5209;
    (t19246, t19249, t19278, t19289, t19298, t19301, t19304)
}
