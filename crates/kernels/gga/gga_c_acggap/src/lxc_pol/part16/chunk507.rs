//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 507/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk507<F: Float>(t2968: F, t228: F, t35: F, t88: F, t223: F, t595: F, t1964: F, t36: F, t265: F, t272: F, t2787: F, t286: F) -> (F, F, F, F, F) {
    let t2969 = F::new(24.0) * t2968;
    let t2970 = t35 * t228;
    let t2971 = t2970 * t88;
    let t2974 = t223 * t595;
    let t2975 = t2974 * t88;
    let t2977 = t36 * t1964;
    let t2978 = t2977 * t88;
    let t2979 = F::new(120.0) * t2978;
    let t2981 = t265 * t2787 * t272;
    let t2982 = t286 * t2981;
    (t2969, t2971, t2975, t2979, t2982)
}
