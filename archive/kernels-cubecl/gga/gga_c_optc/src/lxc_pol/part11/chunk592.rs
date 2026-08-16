//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 592/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk592<F: Float>(t2529: F, t4854: F, t837: F, t845: F, t2480: F, t3640: F, t4770: F, t4774: F, t4778: F, t1354: F) -> (F, F, F, F) {
    let t4856 = t2529 * t4854 * t837;
    let t4858 = F::cast_from(0.11696446794910408142e1_f64) * t845 * t4856;
    let t4863 = t2480 + F::cast_from(0.11415555555555555555e-1_f64) * t3640 - F::cast_from(0.11415555555555555555e-1_f64) * t4770 + F::cast_from(0.34246666666666666666e-1_f64) * t4774 - F::cast_from(0.17123333333333333333e-1_f64) * t4778;
    let t4868 = t1354 * t1354;
    (t4856, t4858, t4863, t4868)
}
