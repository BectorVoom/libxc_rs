//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 847/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk847<F: Float>(t2320: F, t6121: F, t6122: F, t898: F, t6087: F, t6090: F, t6093: F, t6108: F, t378: F, t237: F, t2192: F, t2235: F) -> (F, F, F, F, F, F, F) {
    let t6124 = t6121 * t6122 * t2320;
    let t6126 = F::cast_from(0.10389515463408878255e3_f64) * t898 * t6124;
    let t6127 = F::cast_from(0.28842592592592592592e-1_f64) * t6087;
    let t6131 = -t6127 + F::cast_from(0.37083333333333333334e-1_f64) * t6090 - F::cast_from(0.278125e-1_f64) * t6093 + F::cast_from(0.278125e-1_f64) * t6108;
    let t6132 = t6131 * t378;
    let t6134 = F::cast_from(0.19751673498613801407e-1_f64) * t237 * t6132;
    let t6136 = F::cast_from(3.0_f64) * t2192 * t2235;
    (t6124, t6126, t6127, t6131, t6132, t6134, t6136)
}
