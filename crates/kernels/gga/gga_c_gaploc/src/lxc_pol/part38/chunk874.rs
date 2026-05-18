//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 874/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk874<F: Float>(t13542: F, t731: F, t11613: F, t2508: F, t7659: F, t37032: F, t7663: F, t13489: F, t13495: F, t7137: F, t13486: F, t7129: F) -> (F, F, F, F, F, F) {
    let t44920 = t731 * t13542;
    let t44921 = F::new(0.42725145723012357132e-3) * t44920;
    let t44924 = F::new(0.38452631150711121418e0) * t2508 * t11613 * t7659;
    let t44927 = F::new(0.46143157380853345701e0) * t2508 * t37032 * t7663;
    let t44928 = t731 * t13489;
    let t44931 = F::new(0.10254034973522965712e-1) * t7137 * t13495;
    let t44933 = F::new(0.23071578690426672851e-1) * t7129 * t13486;
    (t44921, t44924, t44927, t44928, t44931, t44933)
}
