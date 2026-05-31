//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1148/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1148<F: Float>(t211: F, t217: F, t22502: F, t23845: F, t23622: F, t23624: F, t23626: F, t23630: F, t23633: F, t23635: F, t23637: F, t23640: F, t23644: F, t23647: F, t23651: F, t23653: F, t23655: F) -> (F, F) {
    let t23913 = F::cast_from(1.0_f64) / t217 / t22502 / t211 / F::cast_from(96.0_f64);
    let t23914 = t23913 * t23845;
    let t23918 = -F::cast_from(0.53675555555555555556e0_f64) * t23622 + F::cast_from(0.40256666666666666668e0_f64) * t23624 + F::cast_from(0.44729629629629629629e0_f64) * t23626 - F::cast_from(0.89459259259259259259e0_f64) * t23630 - F::cast_from(0.301925e0_f64) * t23633 + F::cast_from(0.12524296296296296297e1_f64) * t23635 - F::cast_from(0.16102666666666666667e1_f64) * t23637 + F::cast_from(0.40256666666666666666e1_f64) * t23640 + F::cast_from(0.181155e1_f64) * t23644 + F::cast_from(0.198684e1_f64) * t23647 + F::cast_from(0.49671e0_f64) * t23651 - F::cast_from(0.485484375e1_f64) * t23914 - F::cast_from(0.24154e1_f64) * t23653 + F::cast_from(0.80513333333333333333e0_f64) * t23655;
    (t23914, t23918)
}
