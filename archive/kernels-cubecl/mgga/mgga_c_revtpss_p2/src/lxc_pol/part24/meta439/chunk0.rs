//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1395/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1395<F: Float>(t39768: F, t47065: F, t190: F, t22: F, t519: F, t39762: F, t1317: F, t9545: F, t1340: F, t40129: F, t40182: F, t39821: F) -> (F, F, F, F, F, F, F) {
    let t47067 = F::cast_from(0.19263893255070628431e1_f64) * t47065 * t39768;
    let t47070 = F::cast_from(24.0_f64) * t22 * t519 * t190;
    let t47072 = F::cast_from(0.1301229756036208781e0_f64) * t47065 * t39762;
    let t47073 = t1317 * t9545;
    let t47074 = F::cast_from(16.0_f64) * t47073;
    let t47076 = F::cast_from(0.21053605041484726346e2_f64) * t1340 * t40129;
    let t47084 = F::cast_from(0.5848223622634646207e0_f64) * t1340 * t40182;
    let t47086 = F::cast_from(0.61524113149298439947e4_f64) * t1340 * t39821;
    (t47067, t47070, t47072, t47074, t47076, t47084, t47086)
}
