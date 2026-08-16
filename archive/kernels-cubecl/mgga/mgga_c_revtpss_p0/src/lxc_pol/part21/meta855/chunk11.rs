//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3246/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3246<F: Float>(t45928: F, t45934: F, t45938: F, t45945: F, t45949: F, t2246: F, t4171: F, t10308: F, t1466: F, t13267: F, t602: F, t10326: F, t10355: F, t10356: F, t10368: F, t10373: F, t13299: F, t13302: F, t13303: F, t13306: F, t13312: F, t1469: F, t1480: F, t2251: F, t2258: F, t2270: F, t2275: F, t4186: F, t4201: F, t4202: F, t44: F, t46065: F, t46074: F, t56: F, t606: F, t614: F) -> (F, F, F, F, F, F, F, F, F) {
    let t60214 = F::cast_from(96.0_f64) * t45928;
    let t60215 = F::cast_from(192.0_f64) * t45934;
    let t60216 = F::cast_from(960.0_f64) * t45938;
    let t60217 = F::cast_from(1440.0_f64) * t45945;
    let t60218 = F::cast_from(4032.0_f64) * t45949;
    let t60221 = t4171 * t2246;
    let t60224 = t1466 * t10308;
    let t60248 = t13267 * t602;
    let t60297 = F::cast_from(220.0_f64) / F::cast_from(27.0_f64) * t2270 * t4202 - F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t614 * t13303 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t614 * t13306 + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t614 * t13299 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t44 * t10355 * t4186 * t2251 + F::cast_from(5.0_f64) / F::cast_from(162.0_f64) * t44 * t46065 * t1469 * t10356 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t44 * t2275 * t13312 * t606 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t44 * t13302 * t2258 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t44 * t4201 * t10326 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t1480 * t10373 + F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t56 * t10368 * t4186 * t2251 + F::cast_from(5.0_f64) / F::cast_from(162.0_f64) * t56 * t46074 * t1469 * t10356;
    (t60214, t60215, t60216, t60217, t60218, t60221, t60224, t60248, t60297)
}
