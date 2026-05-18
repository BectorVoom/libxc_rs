//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1042/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1042<F: Float>(t11693: F, t8903: F, t11459: F, t3139: F, t3140: F, t3138: F, t11618: F, t254: F, t906: F, t369: F, t3772: F, t3848: F, t810: F) -> (F, F, F, F, F, F) {
    let t11695 = t8903 * t11693 / F::new(16.0);
    let t11697 = t3139 * t11459 * t3140;
    let t11699 = t3138 * t11697 / F::new(48.0);
    let t11700 = t254 * t11618;
    let t11701 = t11700 * t906;
    let t11706 = t3772 * t369;
    let t11717 = t3848 * t810;
    (t11695, t11697, t11699, t11701, t11706, t11717)
}
