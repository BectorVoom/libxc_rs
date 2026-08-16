//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1322/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1322(t13005: f64, t16771: f64, t20800: f64, t210: f64, t214: f64, t221: f64, t2571: f64, t41155: f64, t41161: f64, t41185: f64, t41200: f64, t4127: f64, t4128: f64, t46764: f64, t46772: f64, t46790: f64, t5544: f64, t68073: f64, t68110: f64, t75978: f64, t76056: f64, t76063: f64, t787: f64) -> f64 {
    let t76359 = t41155 - t41185 - 0.11999999999999999999e0_f64 * t13005 * t221 * t16771 * t5544 + 0.19999999999999999999e-1_f64 * t4127 * t221 * t4128 * t20800 + 0.99999999999999999995e-1_f64 * t41161 * t210 * t214 * t76056 + 0.14999999999999999999e-1_f64 * t2571 * t210 * t214 * t76063 - 0.16666666666666666666e-2_f64 * t787 * t210 * t214 * t75978 - 0.79999999999999999997e-1_f64 * t46764 - 0.13999999999999999999e0_f64 * t68073 + 0.13148148148148148148e0_f64 * t46772 - t41200 - 0.29999999999999999998e-1_f64 * t68110 + 0.22469135802469135801e0_f64 * t46790;
    t76359
}
