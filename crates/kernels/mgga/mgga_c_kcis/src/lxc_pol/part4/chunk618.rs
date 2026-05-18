//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 618/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk618<F: Float>(t313: F, t934: F, t1045: F, t3293: F, t1109: F, t2952: F, t345: F, t1035: F, t346: F, t3074: F, t1114: F, t3096: F) -> (F, F, F, F, F, F, F, F) {
    let t3294 = t313 * t934;
    let t3295 = t3294 * t1045;
    let t3296 = t3293 * t3295;
    let t3299 = t1109 * t2952;
    let t3300 = t345 * t3299;
    let t3303 = t346 * t1035;
    let t3304 = t3303 * t3074;
    let t3305 = t345 * t3304;
    let t3308 = t1114 * t3096;
    (t3295, t3296, t3299, t3300, t3303, t3304, t3305, t3308)
}
