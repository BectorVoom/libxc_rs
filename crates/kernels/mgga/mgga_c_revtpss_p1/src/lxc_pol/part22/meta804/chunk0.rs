//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2906/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2906<F: Float>(t1340: F, t40165: F, t2626: F, t9551: F, t268: F, t520: F, t39768: F, t190: F, t22: F, t519: F, t39762: F, t40129: F) -> (F, F, F, F, F, F) {
    let t47059 = F::cast_from(0.12304822629859687989e5_f64) * t1340 * t40165;
    let t47060 = t9551 * t2626;
    let t47065 = t520 * t268;
    let t47067 = F::cast_from(0.19263893255070628431e1_f64) * t47065 * t39768;
    let t47070 = F::new(24.0) * t22 * t519 * t190;
    let t47072 = F::cast_from(0.1301229756036208781e0_f64) * t47065 * t39762;
    let t47076 = F::cast_from(0.21053605041484726346e2_f64) * t1340 * t40129;
    (t47059, t47060, t47067, t47070, t47072, t47076)
}
