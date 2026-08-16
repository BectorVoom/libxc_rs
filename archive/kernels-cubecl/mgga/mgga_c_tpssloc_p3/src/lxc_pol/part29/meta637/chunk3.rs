//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2094/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2094<F: Float>(t25059: F, t6562: F, t794: F, t82082: F, t82087: F, t1888: F, t25045: F, t82159: F, t7488: F, t82133: F, t25225: F, t6547: F) -> (F, F, F, F, F, F) {
    let t86928 = t6562 * t794 * t25059;
    let t86929 = F::cast_from(0.82246703342411321824e-2_f64) * t86928;
    let t86930 = F::cast_from(0.16449340668482264365e-1_f64) * t82082;
    let t86931 = F::cast_from(0.16449340668482264365e-1_f64) * t82087;
    let t86933 = t1888 * t82159 * t25045;
    let t86940 = t6562 * t82133 * t7488;
    let t86941 = F::cast_from(0.82246703342411321824e-2_f64) * t86940;
    let t86942 = t6547 * t25225;
    (t86929, t86930, t86931, t86933, t86941, t86942)
}
