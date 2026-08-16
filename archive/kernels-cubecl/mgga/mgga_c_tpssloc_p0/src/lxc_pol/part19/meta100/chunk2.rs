//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 557/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk557<F: Float>(t2862: F, t2888: F, t2764: F, t2766: F, t2773: F, t2778: F, t2782: F) -> (F, F) {
    let t2889 = t2862 * t2888;
    let t2892 = F::cast_from(0.12361111111111111111e-1_f64) * t2764;
    let t2897 = t2892 + F::cast_from(0.61805555555555555556e-2_f64) * t2766 - F::cast_from(0.61805555555555555555e-2_f64) * t2773 + F::cast_from(0.18541666666666666667e-1_f64) * t2778 - F::cast_from(0.92708333333333333333e-2_f64) * t2782;
    (t2889, t2897)
}
