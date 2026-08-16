//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3246/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3246(t45928: f64, t45934: f64, t45938: f64, t45945: f64, t45949: f64, t2246: f64, t4171: f64, t10308: f64, t1466: f64, t13267: f64, t602: f64, t10326: f64, t10355: f64, t10356: f64, t10368: f64, t10373: f64, t13299: f64, t13302: f64, t13303: f64, t13306: f64, t13312: f64, t1469: f64, t1480: f64, t2251: f64, t2258: f64, t2270: f64, t2275: f64, t4186: f64, t4201: f64, t4202: f64, t44: f64, t46065: f64, t46074: f64, t56: f64, t606: f64, t614: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t60214 = 96.0_f64 * t45928;
    let t60215 = 192.0_f64 * t45934;
    let t60216 = 960.0_f64 * t45938;
    let t60217 = 1440.0_f64 * t45945;
    let t60218 = 4032.0_f64 * t45949;
    let t60221 = t4171 * t2246;
    let t60224 = t1466 * t10308;
    let t60248 = t13267 * t602;
    let t60297 = 220.0_f64 / 27.0_f64 * t2270 * t4202 - 40.0_f64 / 9.0_f64 * t614 * t13303 - 20.0_f64 / 9.0_f64 * t614 * t13306 + 10.0_f64 / 27.0_f64 * t614 * t13299 - 5.0_f64 / 36.0_f64 * t44 * t10355 * t4186 * t2251 + 5.0_f64 / 162.0_f64 * t44 * t46065 * t1469 * t10356 + 5.0_f64 / 6.0_f64 * t44 * t2275 * t13312 * t606 + 5.0_f64 / 6.0_f64 * t44 * t13302 * t2258 + 5.0_f64 / 18.0_f64 * t44 * t4201 * t10326 - 20.0_f64 / 9.0_f64 * t1480 * t10373 + 5.0_f64 / 36.0_f64 * t56 * t10368 * t4186 * t2251 + 5.0_f64 / 162.0_f64 * t56 * t46074 * t1469 * t10356;
    (t60214, t60215, t60216, t60217, t60218, t60221, t60224, t60248, t60297)
}
