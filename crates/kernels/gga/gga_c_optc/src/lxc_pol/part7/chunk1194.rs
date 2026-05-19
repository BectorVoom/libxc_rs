//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1194/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1194<F: Float>(t23620: F, t23622: F, t23624: F, t23626: F, t23630: F, t23633: F, t23635: F, t23637: F, t23640: F, t23644: F, t23660: F, t24678: F) -> F {
    let t24690 = t24678 - F::cast_from(0.47488888888888888888e-1_f64) * t23620 - F::cast_from(0.31659259259259259258e-1_f64) * t23622 + F::cast_from(0.23744444444444444444e-1_f64) * t23624 + F::cast_from(0.26382716049382716049e-1_f64) * t23626 - F::cast_from(0.52765432098765432099e-1_f64) * t23630 - F::cast_from(0.17808333333333333333e-1_f64) * t23633 + F::cast_from(0.73871604938271604937e-1_f64) * t23635 - F::cast_from(0.94977777777777777776e-1_f64) * t23637 + F::cast_from(0.23744444444444444444e0_f64) * t23640 + F::new(0.10685e0) * t23644 + F::cast_from(0.14246666666666666667e0_f64) * t23660;
    t24690
}
