//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 963/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk963<F: Float>(t2412: F, t7687: F, t1356: F, t35731: F, t35737: F, t35742: F, t35744: F, t35752: F, t35766: F, t36288: F, t40480: F, t40481: F, t40489: F, t40491: F, t40493: F, t4601: F, t5019: F, t5144: F, t5267: F, t5888: F, t739: F, t7567: F, t8393: F, t8396: F, t884: F) -> F {
    let t40495 = t2412 * t7687;
    let t40497 = -F::cast_from(0.30487649791575028314e-3_f64) * t35731 - F::cast_from(0.15243824895787514157e-3_f64) * t35737 + F::cast_from(0.30487649791575028314e-3_f64) * t35742 + F::cast_from(0.30487649791575028314e-3_f64) * t35744 + F::cast_from(0.23948483403727617128e0_f64) * t35752 + F::cast_from(0.23948483403727617128e0_f64) * t739 * t7567 * t5144 - F::cast_from(0.23948483403727617128e0_f64) * t884 * t7567 * t5267 - F::cast_from(0.23948483403727617128e0_f64) * t1356 * t36288 * t5888 + F::cast_from(0.79828278012425390426e-1_f64) * t35766 + t40480 + F::cast_from(0.85129199786595678796e-5_f64) * t40481 - F::cast_from(0.47896966807455234256e0_f64) * t5019 * t8396 + F::cast_from(0.35922725105591425692e0_f64) * t4601 * t8393 + F::cast_from(0.13637330827122670864e-1_f64) * t40489 - F::cast_from(0.20455996240684006296e-1_f64) * t40491 + F::cast_from(0.27274661654245341728e-1_f64) * t40493 - F::cast_from(0.42564599893297839398e-5_f64) * t40495;
    t40497
}
