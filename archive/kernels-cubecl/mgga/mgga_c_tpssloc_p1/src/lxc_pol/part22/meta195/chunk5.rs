//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1149/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1149<F: Float>(t2892: F, t4335: F, t5679: F, t5683: F, t5687: F) -> F {
    let t5769 = t2892 + F::cast_from(0.61805555555555555556e-2_f64) * t4335 - F::cast_from(0.61805555555555555555e-2_f64) * t5679 + F::cast_from(0.18541666666666666667e-1_f64) * t5683 - F::cast_from(0.92708333333333333333e-2_f64) * t5687;
    t5769
}
