//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1437/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1437<F: Float>(t4574: F, t537: F, t11353: F, t9507: F, t11360: F, t2884: F, t11348: F, t11605: F, t22943: F, t2821: F, t2829: F, t2853: F, t2859: F, t30841: F, t30871: F, t30892: F, t30895: F, t30903: F, t30975: F, t3680: F, t3733: F, t4494: F, t7643: F, t7806: F, t9632: F, t9639: F, t9654: F, tau1: F) -> (F, F, F, F) {
    let t31207 = t4574 * t537;
    let t31222 = t11353 * t9507;
    let t31225 = t11360 * t9507;
    let t31228 = t2884 * tau1;
    let t31229 = t31228 * t11348;
    let t31236 = -F::cast_from(64.0_f64) / F::cast_from(81.0_f64) * t31207 * t2853 - F::cast_from(32.0_f64) / F::cast_from(27.0_f64) * t11605 * t2859 - F::cast_from(32.0_f64) / F::cast_from(27.0_f64) * t2821 * t30892 + F::cast_from(32.0_f64) / F::cast_from(27.0_f64) * t2829 * t30895 - F::cast_from(64.0_f64) / F::cast_from(27.0_f64) * t3680 * t30903 - F::cast_from(64.0_f64) / F::cast_from(81.0_f64) * t3733 * t30841 + F::cast_from(32.0_f64) * t7806 * t30871 + F::cast_from(704.0_f64) / F::cast_from(27.0_f64) * t9632 * t31222 - F::cast_from(1408.0_f64) / F::cast_from(81.0_f64) * t9654 * t31225 - F::cast_from(6400.0_f64) / F::cast_from(81.0_f64) * t9639 * t31229 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t22943 * t4494 - F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t7643 * t30975;
    (t31222, t31225, t31229, t31236)
}
