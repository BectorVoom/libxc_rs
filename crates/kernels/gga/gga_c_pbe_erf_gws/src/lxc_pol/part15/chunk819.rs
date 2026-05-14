//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 819/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk819<F: Float>(t1730: F, t2753: F, t1: F, t837: F, t2736: F, t616: F, t5459: F, t5465: F, t2608: F, t5493: F, t1620: F, t1724: F, t2607: F, t1621: F, t2637: F, t7136: F) -> (F, F, F, F, F, F, F, F) {
    let t7775 = 16.0 / 45.0 * t1730 * t2753;
    let t7776 = t1 * t837;
    let t7777 = t7776 * t2736;
    let t7778 = t616 * t7777;
    let t7779 = 4.0 / 9.0 * t7778;
    let t7780 = 16.0 / 135.0 * t5459;
    let t7781 = 16.0 / 405.0 * t5465;
    let t7782 = t5493 * t2608;
    let t7784 = 16.0 / 45.0 * t1620 * t7782;
    let t7785 = t2607 * t1724;
    let t7786 = t1621 * t7785;
    let t7788 = 4.0 / 15.0 * t1620 * t7786;
    let t7790 = 8.0 / 15.0 * t7136 * t2637;
    (t7775, t7776, t7779, t7780, t7781, t7784, t7788, t7790)
}
