//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1066/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1066<F: Float>(t898: F, t9688: F, t338: F, t353: F, t2246: F, t3099: F, t2118: F, t8652: F, t3074: F, t3202: F, t840: F, t3306: F, t810: F) -> (F, F, F, F, F, F) {
    let t9689 = t898 * t9688;
    let t9691 = t338 * t353 * t9689;
    let t9695 = F::new(7.0) / F::new(72.0) * t2246 * t3099;
    let t9696 = t2118 * t8652;
    let t9697 = t3074 * t9696;
    let t9701 = F::new(7.0) / F::new(144.0) * t840 * t3202;
    let t9702 = t3306 * t810;
    (t9689, t9691, t9695, t9697, t9701, t9702)
}
