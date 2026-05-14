//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 852/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk852<F: Float>(t2186: F, t8597: F, t2412: F, t7404: F, t352: F, t8924: F, t262: F, t8620: F, t34735: F, t8902: F, t36639: F, t8906: F, t7687: F, t1356: F, t35731: F, t35737: F, t35742: F, t35744: F, t35752: F, t35766: F, t36288: F, t4601: F, t5019: F, t5144: F, t5267: F, t5888: F, t739: F, t7567: F, t8393: F, t8396: F, t884: F) -> (F, F, F) {
    let t40479 = t2186 * t8597;
    let t40480 = 0.19863479950205658386e-4 * t40479;
    let t40481 = t2412 * t7404;
    let t40487 = t8924 * t352;
    let t40488 = t262 * t40487;
    let t40489 = t8620 * t40488;
    let t40491 = t34735 * t8902;
    let t40493 = t36639 * t8906;
    let t40495 = t2412 * t7687;
    let t40497 = -0.30487649791575028314e-3 * t35731 - 0.15243824895787514157e-3 * t35737 + 0.30487649791575028314e-3 * t35742 + 0.30487649791575028314e-3 * t35744 + 0.23948483403727617128e0 * t35752 + 0.23948483403727617128e0 * t739 * t7567 * t5144 - 0.23948483403727617128e0 * t884 * t7567 * t5267 - 0.23948483403727617128e0 * t1356 * t36288 * t5888 + 0.79828278012425390426e-1 * t35766 + t40480 + 0.85129199786595678796e-5 * t40481 - 0.47896966807455234256e0 * t5019 * t8396 + 0.35922725105591425692e0 * t4601 * t8393 + 0.13637330827122670864e-1 * t40489 - 0.20455996240684006296e-1 * t40491 + 0.27274661654245341728e-1 * t40493 - 0.42564599893297839398e-5 * t40495;
    (t40487, t40488, t40497)
}
