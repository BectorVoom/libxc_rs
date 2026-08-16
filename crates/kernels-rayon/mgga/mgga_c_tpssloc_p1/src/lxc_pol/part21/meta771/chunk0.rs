//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2672/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2672(t39328: f64, t39330: f64, t39339: f64, t39341: f64, t54323: f64, t54325: f64, t16153: f64, t19631: f64, t3918: f64, t3919: f64, t39338: f64, t39346: f64, t39349: f64, t39356: f64, t5122: f64, t5126: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t56149 = 0.32530743900905219526e-1_f64 * t39328;
    let t56150 = 0.24415263074675393405e-3_f64 * t39330;
    let t56151 = 0.70178683471615754484e1_f64 * t39339;
    let t56152 = 0.10389515463408878255e3_f64 * t39341;
    let t56159 = 8.0_f64 * t54323;
    let t56160 = 0.11393789434848516923e-2_f64 * t54325;
    let t56161 = 12.0_f64 * t16153 * t5122 * t5126 + 6.0_f64 * t19631 * t3918 * t3919 - t39338 + t39346 + t39349 + t39356 + t56149 + t56150 + t56151 - t56152 - t56159 - t56160;
    (t56149, t56150, t56151, t56152, t56159, t56160, t56161)
}
