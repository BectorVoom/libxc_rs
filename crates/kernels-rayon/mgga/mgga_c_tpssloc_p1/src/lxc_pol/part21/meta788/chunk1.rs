//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2746/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2746(t40682: f64, t40687: f64, t46196: f64, t1484: f64, t2752: f64, t13487: f64, t2749: f64, t12854: f64, t12915: f64, t13196: f64, t1530: f64, t16596: f64, t16944: f64, t17116: f64, t17120: f64, t1877: f64, t193: f64, t200: f64, t2522: f64, t2523: f64, t2745: f64, t39373: f64, t40685: f64, t4310: f64, t4314: f64) -> (f64, f64, f64, f64) {
    let t57903 = 0.70178683471615754484e1_f64 * t40682;
    let t57907 = 4.0_f64 * t40687;
    let t57908 = 0.70178683471615754484e1_f64 * t46196;
    let t57911 = t2752 * t1484;
    let t57912 = t57911 * t13487;
    let t57921 = t1484 * t2749;
    let t57931 = -24.0_f64 * t1530 * t193 * t200 * t57912 - 12.0_f64 * t12854 * t16596 * t2522 + 12.0_f64 * t12915 * t2522 * t57921 + 12.0_f64 * t13196 * t4310 * t4314 - 6.0_f64 * t13487 * t17116 * t2522 + 24.0_f64 * t16944 * t2523 * t4314 + 2.0_f64 * t17120 * t1877 * t2745 + t39373 - t40685 + t57903 + t57907 + t57908;
    (t57903, t57907, t57908, t57931)
}
