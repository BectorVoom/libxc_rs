//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 737/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk737<F: Float>(t2508: F, t44712: F, t688: F, t779: F, t1897: F, t35573: F, t954: F, t3009: F, t32356: F, t7226: F, t13542: F, t731: F, t11613: F, t7659: F, t37032: F, t7663: F) -> (F, F, F, F, F, F) {
    let t44905 = 0.76905262301422242837e-2 * t2508 * t779 * t44712 * t688;
    let t44912 = 0.76905262301422242837e-2 * t1897 * t954 * t35573;
    let t44916 = 0.92286314761706691402e-1 * t2508 * t7226 * t3009 * t32356;
    let t44920 = t731 * t13542;
    let t44921 = 0.42725145723012357132e-3 * t44920;
    let t44924 = 0.38452631150711121418e0 * t2508 * t11613 * t7659;
    let t44927 = 0.46143157380853345701e0 * t2508 * t37032 * t7663;
    (t44905, t44912, t44916, t44921, t44924, t44927)
}
