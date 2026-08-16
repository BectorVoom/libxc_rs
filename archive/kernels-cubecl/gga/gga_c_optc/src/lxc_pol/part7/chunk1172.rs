//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1172/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1172<F: Float>(t23682: F, t23620: F, t23622: F, t23624: F, t23626: F, t23630: F, t23633: F, t23635: F, t23637: F, t23640: F, t23644: F, t23660: F) -> F {
    let t24321 = F::cast_from(0.96141975308641975307e-1_f64) * t23682;
    let t24333 = t24321 - F::cast_from(0.24722222222222222222e-1_f64) * t23620 - F::cast_from(0.16481481481481481482e-1_f64) * t23622 + F::cast_from(0.12361111111111111111e-1_f64) * t23624 + F::cast_from(0.13734567901234567901e-1_f64) * t23626 - F::cast_from(0.27469135802469135803e-1_f64) * t23630 - F::cast_from(0.92708333333333333333e-2_f64) * t23633 + F::cast_from(0.38456790123456790123e-1_f64) * t23635 - F::cast_from(0.49444444444444444444e-1_f64) * t23637 + F::cast_from(0.12361111111111111111e0_f64) * t23640 + F::cast_from(0.55625000000000000001e-1_f64) * t23644 + F::cast_from(0.74166666666666666668e-1_f64) * t23660;
    t24333
}
