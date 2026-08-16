//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1224/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1224(t84130: f64, t84322: f64, t84719: f64, t85370: f64, t12521: f64, t12524: f64, t12529: f64, t12532: f64, t1401: f64, t16535: f64, t20173: f64, t2039: f64, t2098: f64, t2319: f64, t2363: f64, t23917: f64, t24462: f64, t24465: f64, t24478: f64, t24481: f64, t3938: f64, t3941: f64, t45557: f64, t45560: f64, t55344: f64, t577: f64, t671: f64, t7056: f64, t7230: f64, t7235: f64, t84033: f64, t84044: f64, t84078: f64, t9416: f64) -> (f64, f64) {
    let t85372 = t84130 + t84322 + t84719 + t85370;
    let t85375 = 81.0_f64 * t84033 * t2319 + 0.135e2_f64 * t1401 * t84044 + 81.0_f64 * t16535 * t7056 + 81.0_f64 * t3941 * t23917 * t671 + 81.0_f64 * t3941 * t7056 * t2363 + 81.0_f64 * t55344 * t2039 + 162.0_f64 * t12524 * t24478 + 81.0_f64 * t12524 * t24481 + 81.0_f64 * t45560 * t7235 + 81.0_f64 * t20173 * t24481 + 0.405e2_f64 * t12521 * t7056 + 0.405e2_f64 * t3938 * t23917 + 0.135e2_f64 * t7230 * t9416 + 0.135e2_f64 * t45557 * t2039 + 27.0_f64 * t2098 * t12529 + 27.0_f64 * t3941 * t2039 * t9416 + 0.405e2_f64 * t84078 * t671 + 81.0_f64 * t24465 * t12532 + 0.405e2_f64 * t24462 * t2363 + 0.45e1_f64 * t85372 * t577;
    (t85372, t85375)
}
