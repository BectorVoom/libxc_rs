//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 527/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk527<F: Float>(t1: F, t901: F, t283: F, t2868: F, t88: F, t228: F, t35: F, t223: F, t595: F, t1964: F, t36: F, t265: F, t272: F, t2787: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2965 = t901 * t1;
    let t2966 = t2965 * t283;
    let t2968 = t2868 * t88;
    let t2969 = F::cast_from(24.0_f64) * t2968;
    let t2970 = t35 * t228;
    let t2971 = t2970 * t88;
    let t2974 = t223 * t595;
    let t2975 = t2974 * t88;
    let t2977 = t36 * t1964;
    let t2978 = t2977 * t88;
    let t2979 = F::cast_from(120.0_f64) * t2978;
    let t2981 = t265 * t2787 * t272;
    (t2965, t2966, t2969, t2970, t2971, t2974, t2975, t2977, t2978, t2979, t2981)
}
