//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2305/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2305(t46196: f64, t21066: f64, t870: f64, t5544: f64, t868: f64, t57947: f64, t5527: f64, t57960: f64, t46208: f64, t17116: f64, t1877: f64, t20947: f64, t2522: f64, t2523: f64, t39411: f64, t40714: f64, t40716: f64, t4303: f64, t4307: f64, t4314: f64, t46207: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t67105 = 0.10526802520742363173e2_f64 * t46196;
    let t67112 = t21066 * t870;
    let t67123 = t5544 * t868;
    let t67127 = 12.0_f64 * t57947;
    let t67128 = t5527 * t868;
    let t67132 = 24.0_f64 * t57960;
    let t67133 = 0.30762056574649219972e4_f64 * t46208;
    let t67134 = -3.0_f64 * t17116 * t1877 * t4303 + 18.0_f64 * t20947 * t2523 * t4314 - 9.0_f64 * t2522 * t4307 * t67123 - 18.0_f64 * t4307 * t4314 * t67128 + t39411 - t40714 + t40716 + t46207 + t67127 + t67132 - t67133;
    (t67105, t67112, t67127, t67132, t67133, t67134)
}
