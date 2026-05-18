//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 965/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk965<F: Float>(t10770: F, t2508: F, t2927: F, t954: F, t3448: F, t7137: F, t8440: F, t977: F, t2728: F, t2969: F, t3459: F, t841: F) -> (F, F, F, F, F, F, F) {
    let t10772 = F::new(0.76905262301422242837e-2) * t2508 * t10770;
    let t10773 = t954 * t2927;
    let t10775 = F::new(0.76905262301422242837e-2) * t2508 * t10773;
    let t10788 = F::new(0.20508069947045931423e-1) * t7137 * t3448;
    let t10797 = t8440 * t977;
    let t10798 = t2969 * t2728;
    let t10802 = t3459 * t841;
    (t10772, t10773, t10775, t10788, t10797, t10798, t10802)
}
