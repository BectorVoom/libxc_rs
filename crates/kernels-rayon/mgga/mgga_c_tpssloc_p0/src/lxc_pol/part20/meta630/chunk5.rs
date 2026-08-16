//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2289/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2289(t47276: f64, t13176: f64, t2696: f64, t849: f64, t13222: f64, t13228: f64, t13251: f64, t13300: f64, t13306: f64, t13350: f64, t2643: f64, t2645: f64, t2647: f64, t2679: f64, t41063: f64, t41090: f64, t4178: f64, t4248: f64, t4250: f64, t47012: f64, t47262: f64, t47267: f64, t47270: f64, t47271: f64, t47273: f64, t9627: f64, t9642: f64, t9653: f64, t9958: f64) -> f64 {
    let t47277 = 119.0_f64 / 1152.0_f64 * t47276;
    let t47278 = t13176 * t2696;
    let t47279 = t47278 * t849;
    let t47281 = t2643 * t2645 * t13300 * t2679 / 256.0_f64 + t9642 * t13306 / 256.0_f64 + t2643 * t2645 * t4248 * t9958 / 768.0_f64 + t41063 * t4250 / 256.0_f64 + t13251 * t9653 / 256.0_f64 - t4178 * t13222 * t13228 * t41090 / 128.0_f64 + 5.0_f64 / 128.0_f64 * t4178 * t13350 * t47012 * t9627 + t2643 * t13222 * t47262 * t2647 / 256.0_f64 - 35.0_f64 / 384.0_f64 * t47267 - t47270 + 7.0_f64 / 384.0_f64 * t47271 + 7.0_f64 / 192.0_f64 * t47273 - t47277 + 7.0_f64 / 192.0_f64 * t47279;
    t47281
}
