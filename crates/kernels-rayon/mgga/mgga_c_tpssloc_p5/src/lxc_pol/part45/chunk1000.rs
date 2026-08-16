//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1000/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1000(t23909: f64, t8526: f64, t23918: f64, t1873: f64, t24428: f64, t652: f64, t112528: f64, t112535: f64, t112537: f64, t112542: f64, t114573: f64, t115195: f64, t115208: f64, t1976: f64, t2039: f64, t22483: f64, t2314: f64, t2364: f64, t23829: f64, t23933: f64, t23941: f64, t31532: f64, t31726: f64, t31734: f64, t4034: f64, t6517: f64, t7042: f64, t8529: f64, t9348: f64) -> f64 {
    let t115210 = 2.0_f64 * t8526 * t23909;
    let t115212 = 2.0_f64 * t8526 * t23918;
    let t115217 = 2.0_f64 * t652 * t24428 * t1873;
    let t115222 = -2.0_f64 * t2039 * t23829 * t652 - 2.0_f64 * t1976 * t23941 - 2.0_f64 * t22483 * t7042 - 4.0_f64 * t2314 * t31726 - 2.0_f64 * t2364 * t31532 - 4.0_f64 * t23933 * t6517 - 4.0_f64 * t31734 * t4034 - 2.0_f64 * t8529 * t9348 - t112528 - t112535 - t112537 - t112542 - t114573 - t115195 - t115208 - t115210 - t115212 - t115217;
    t115222
}
