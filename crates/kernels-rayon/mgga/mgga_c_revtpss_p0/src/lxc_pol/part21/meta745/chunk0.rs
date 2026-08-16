//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2619/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2619(t3857: f64, t5567: f64, t1317: f64, t13672: f64, t2608: f64, t512: f64, t5566: f64, t1856: f64, t9544: f64, t46975: f64, t39483: f64, t39520: f64, t39528: f64, t39531: f64, t46970: f64, t48223: f64, t48224: f64, t48226: f64, t48228: f64, t48231: f64, t48232: f64, t48234: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48235 = t3857 * t5567;
    let t48236 = 60.0_f64 * t48235;
    let t48237 = t1317 * t13672;
    let t48238 = 12.0_f64 * t48237;
    let t48240 = t512 * t5566 * t2608;
    let t48241 = 3.0_f64 * t48240;
    let t48243 = t512 * t1856 * t9544;
    let t48244 = 240.0_f64 * t46975;
    let t48245 = t46970 - t48223 + t48224 - t39483 + t48226 + t39520 + t48228 + t48231 - t39528 - t48232 + t39531 - t48234 + t48236 + t48238 + t48241 + t48243 + t48244;
    (t48236, t48238, t48241, t48243, t48244, t48245)
}
