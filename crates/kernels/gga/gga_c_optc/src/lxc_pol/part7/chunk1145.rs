//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1145/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1145<F: Float>(t23682: F, t23620: F, t23622: F, t23624: F, t23626: F, t23630: F, t23633: F, t23635: F, t23637: F, t23640: F, t23644: F, t23660: F) -> F {
    let t23860 = F::cast_from(280.0_f64) / F::cast_from(81.0_f64) * t23682;
    let t23872 = t23860 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t23620 - F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t23622 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t23624 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t23626 - F::cast_from(80.0_f64) / F::cast_from(81.0_f64) * t23630 - t23633 / F::cast_from(3.0_f64) + F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t23635 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t23637 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t23640 + F::cast_from(2.0_f64) * t23644 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t23660;
    t23872
}
