//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 1025/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk1025(t10105: f64, t1052: f64, t1960: f64, t3418: f64, t6553: f64, t10283: f64, t2497: f64, t42520: f64, t42523: f64, t42904: f64, t44215: f64, t44217: f64, t44221: f64, t44223: f64, t44225: f64, t44228: f64, t44231: f64, t44232: f64, t44234: f64, t44236: f64, t44238: f64, t44239: f64) -> (f64, f64, f64) {
    let t44242 = 2.0_f64 * t1960 * t1052 * t10105;
    let t44243 = t6553 * t3418;
    let t44244 = 2.0_f64 * t44243;
    let t44245 = t10283 * t2497;
    let t44246 = 2.0_f64 * t44245;
    let t44247 = 4.0_f64 * t44215 + 4.0_f64 * t44217 - t44221 + t44223 + t44225 - t44228 + t42520 + t44231 - t42523 - t44232 - t44234 - t42904 + t44236 + t44238 - t44239 + t44242 + t44244 + t44246;
    (t44244, t44246, t44247)
}
