//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1006/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1006<F: Float>(t3696: F, t3703: F, t424: F, t134: F, t3698: F, t3702: F, t11534: F, t1026: F, t632: F, t3018: F, t3022: F, t3691: F) -> (F, F, F, F, F, F, F) {
    let t11555 = t424 * t3696 * t3703;
    let t11557 = t3698 * t134;
    let t11558 = t11557 * t3702;
    let t11559 = t11534 * t11558;
    let t11561 = t632 * t1026;
    let t11562 = t11561 * t3018;
    let t11564 = t3691 * t3022;
    (t11555, t11557, t11558, t11559, t11561, t11562, t11564)
}
