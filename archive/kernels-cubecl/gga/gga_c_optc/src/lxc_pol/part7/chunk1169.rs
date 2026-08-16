//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1169/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1169<F: Float>(t23622: F, t23624: F, t23626: F, t23630: F, t23633: F, t23635: F, t23637: F, t23640: F, t23644: F, t23647: F, t23651: F, t23653: F, t23655: F, t23914: F) -> F {
    let t24279 = -F::cast_from(0.5314962962962962963e0_f64) * t23622 + F::cast_from(0.39862222222222222223e0_f64) * t23624 + F::cast_from(0.44291358024691358024e0_f64) * t23626 - F::cast_from(0.88582716049382716048e0_f64) * t23630 - F::cast_from(0.29896666666666666667e0_f64) * t23633 + F::cast_from(0.12401580246913580247e1_f64) * t23635 - F::cast_from(0.15944888888888888889e1_f64) * t23637 + F::cast_from(0.39862222222222222223e1_f64) * t23640 + F::cast_from(0.17938e1_f64) * t23644 + F::cast_from(0.197176e1_f64) * t23647 + F::cast_from(0.49293999999999999999e0_f64) * t23651 - F::cast_from(0.3560484375e1_f64) * t23914 - F::cast_from(0.23917333333333333333e1_f64) * t23653 + F::cast_from(0.79724444444444444444e0_f64) * t23655;
    t24279
}
