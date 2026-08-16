//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1186/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1186(t40041: f64, t562: f64, t12168: f64, t12172: f64, t12179: f64, t12240: f64, t12241: f64, t12256: f64, t12267: f64, t12273: f64, t1336: f64, t1380: f64, t16033: f64, t16047: f64, t16055: f64, t22740: f64, t3777: f64, t3901: f64, t3905: f64, t40271: f64, t40335: f64, t40439: f64, t5334: f64, t564: f64) -> (f64, f64) {
    let t40541 = t40041 * t562;
    let t40576 = -4.0_f64 * t12168 * t1336 * t3901 + 36.0_f64 * t12240 * t22740 * t5334 - t1336 * t1380 * t40271 - 36.0_f64 * t16047 * t22740 * t40335 + 24.0_f64 * t12172 * t3777 - 4.0_f64 * t12179 * t3777 + 24.0_f64 * t12241 * t16055 + 24.0_f64 * t12256 * t3777 - 6.0_f64 * t12267 * t3905 - 12.0_f64 * t12273 * t16033 + t40439 * t564;
    (t40541, t40576)
}
