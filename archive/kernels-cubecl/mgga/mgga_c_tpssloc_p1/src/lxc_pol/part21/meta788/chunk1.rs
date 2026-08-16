//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2746/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2746<F: Float>(t40682: F, t40687: F, t46196: F, t1484: F, t2752: F, t13487: F, t2749: F, t12854: F, t12915: F, t13196: F, t1530: F, t16596: F, t16944: F, t17116: F, t17120: F, t1877: F, t193: F, t200: F, t2522: F, t2523: F, t2745: F, t39373: F, t40685: F, t4310: F, t4314: F) -> (F, F, F, F) {
    let t57903 = F::cast_from(0.70178683471615754484e1_f64) * t40682;
    let t57907 = F::cast_from(4.0_f64) * t40687;
    let t57908 = F::cast_from(0.70178683471615754484e1_f64) * t46196;
    let t57911 = t2752 * t1484;
    let t57912 = t57911 * t13487;
    let t57921 = t1484 * t2749;
    let t57931 = -F::cast_from(24.0_f64) * t1530 * t193 * t200 * t57912 - F::cast_from(12.0_f64) * t12854 * t16596 * t2522 + F::cast_from(12.0_f64) * t12915 * t2522 * t57921 + F::cast_from(12.0_f64) * t13196 * t4310 * t4314 - F::cast_from(6.0_f64) * t13487 * t17116 * t2522 + F::cast_from(24.0_f64) * t16944 * t2523 * t4314 + F::cast_from(2.0_f64) * t17120 * t1877 * t2745 + t39373 - t40685 + t57903 + t57907 + t57908;
    (t57903, t57907, t57908, t57931)
}
