//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 613/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk613<F: Float>(t10773: F, t2508: F, t3448: F, t7137: F, t3459: F, t841: F, t1052: F, t2728: F, t1022: F, t830: F, t1: F, t787: F) -> (F, F, F, F, F, F, F) {
    let t10775 = F::new(0.76905262301422242837e-2) * t2508 * t10773;
    let t10788 = F::new(0.20508069947045931423e-1) * t7137 * t3448;
    let t10802 = t3459 * t841;
    let t10805 = t1052 * t2728;
    let t10809 = t830 * t1022;
    let t10810 = t10809 * t1;
    let t10811 = t787 * t10810;
    (t10775, t10788, t10802, t10805, t10809, t10810, t10811)
}
