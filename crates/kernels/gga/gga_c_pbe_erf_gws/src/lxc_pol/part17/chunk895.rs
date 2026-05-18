//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 895/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk895<F: Float>(t1730: F, t2753: F, t1: F, t837: F, t2736: F, t616: F, t5459: F, t5465: F, t2608: F, t5493: F, t1620: F, t1724: F, t2607: F) -> (F, F, F, F, F, F, F) {
    let t7775 = F::new(16.0) / F::new(45.0) * t1730 * t2753;
    let t7776 = t1 * t837;
    let t7777 = t7776 * t2736;
    let t7778 = t616 * t7777;
    let t7779 = F::new(4.0) / F::new(9.0) * t7778;
    let t7780 = F::new(16.0) / F::new(135.0) * t5459;
    let t7781 = F::new(16.0) / F::new(405.0) * t5465;
    let t7782 = t5493 * t2608;
    let t7784 = F::new(16.0) / F::new(45.0) * t1620 * t7782;
    let t7785 = t2607 * t1724;
    (t7775, t7776, t7779, t7780, t7781, t7784, t7785)
}
