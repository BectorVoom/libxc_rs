//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1158/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1158<F: Float>(t20693: F, t20698: F, t822: F, t6253: F, t6563: F, t2100: F, t816: F, t2074: F, t2157: F, t2170: F, t3138: F, t6177: F) -> (F, F, F, F, F) {
    let t20700 = t822 * t20693 * t20698 / F::new(16.0);
    let t20702 = F::new(3.0) / F::new(8.0) * t6253 * t6563;
    let t20703 = t816 * t2100;
    let t20708 = t2157 * t2074;
    let t20712 = t3138 * t2170 * t6177 * t20708 / F::new(4.0);
    (t20700, t20702, t20703, t20708, t20712)
}
