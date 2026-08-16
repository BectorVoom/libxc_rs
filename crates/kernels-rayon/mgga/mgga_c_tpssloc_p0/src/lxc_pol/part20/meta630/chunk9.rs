//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2293/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2293(t2627: f64, t4265: f64, t226: f64, t40931: f64, t68: f64, t13377: f64, t814: f64, t10073: f64, t10081: f64, t13176: f64, t13380: f64, t13397: f64, t13416: f64, t13423: f64, t2617: f64, t2633: f64, t2736: f64, t4166: f64, t4281: f64, t4282: f64, t4288: f64, t47308: f64, t812: f64, t829: f64, t9612: f64, t9976: f64, t9981: f64) -> f64 {
    let t47374 = t2627 * t4265;
    let t47386 = t226 * t68 * t40931;
    let t47395 = t814 * t13377;
    let t47399 = 18.0_f64 * t13380 * t2633 * t4281 - 36.0_f64 * t13397 * t4282 * t9976 + 6.0_f64 * t13416 * t812 * t9981 + 6.0_f64 * t2633 * t47374 * t812 + 24.0_f64 * t4282 * t47308 * t47386 - 3.0_f64 * t47395 * t812 * t829 - 3.0_f64 * t10073 * t4166 - 6.0_f64 * t10081 * t4166 - 3.0_f64 * t13176 * t2736 - 3.0_f64 * t13423 * t2617 - 3.0_f64 * t4288 * t9612;
    t47399
}
