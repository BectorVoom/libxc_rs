//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 963/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk963(t2412: f64, t7687: f64, t1356: f64, t35731: f64, t35737: f64, t35742: f64, t35744: f64, t35752: f64, t35766: f64, t36288: f64, t40480: f64, t40481: f64, t40489: f64, t40491: f64, t40493: f64, t4601: f64, t5019: f64, t5144: f64, t5267: f64, t5888: f64, t739: f64, t7567: f64, t8393: f64, t8396: f64, t884: f64) -> f64 {
    let t40495 = t2412 * t7687;
    let t40497 = -0.30487649791575028314e-3_f64 * t35731 - 0.15243824895787514157e-3_f64 * t35737 + 0.30487649791575028314e-3_f64 * t35742 + 0.30487649791575028314e-3_f64 * t35744 + 0.23948483403727617128e0_f64 * t35752 + 0.23948483403727617128e0_f64 * t739 * t7567 * t5144 - 0.23948483403727617128e0_f64 * t884 * t7567 * t5267 - 0.23948483403727617128e0_f64 * t1356 * t36288 * t5888 + 0.79828278012425390426e-1_f64 * t35766 + t40480 + 0.85129199786595678796e-5_f64 * t40481 - 0.47896966807455234256e0_f64 * t5019 * t8396 + 0.35922725105591425692e0_f64 * t4601 * t8393 + 0.13637330827122670864e-1_f64 * t40489 - 0.20455996240684006296e-1_f64 * t40491 + 0.27274661654245341728e-1_f64 * t40493 - 0.42564599893297839398e-5_f64 * t40495;
    t40497
}
