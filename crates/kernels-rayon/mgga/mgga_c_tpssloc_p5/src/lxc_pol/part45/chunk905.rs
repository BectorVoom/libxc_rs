//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 905/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk905(t1388: f64, t2018: f64, t26558: f64, t26161: f64, t1393: f64, t1869: f64, t2075: f64, t2096: f64, t2314: f64, t31246: f64, t31753: f64, t31761: f64, t31769: f64, t31771: f64, t31774: f64, t6515: f64, t6539: f64, t7042: f64, t7156: f64, t7218: f64, t7220: f64, t8450: f64, t8529: f64, t8604: f64) -> (f64, f64, f64) {
    let t31775 = t2018 * t1388;
    let t31776 = t26558 * t31775;
    let t31778 = 2.0_f64 * t26161 * t31776;
    let t31779 = t1393 * t8604 - t1869 * t7156 - t2075 * t6515 + t2096 * t31246 - 2.0_f64 * t2314 * t8529 - 2.0_f64 * t6539 * t7042 + t7218 * t8450 - t7220 * t8450 - t31753 + t31761 - t31769 - t31771 - t31774 + t31778;
    (t31775, t31776, t31779)
}
