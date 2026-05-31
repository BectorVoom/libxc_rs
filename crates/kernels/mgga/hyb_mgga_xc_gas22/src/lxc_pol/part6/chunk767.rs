//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 767/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk767<F: Float>(t2234: F, t4143: F, t2240: F, t3300: F, t4106: F, t1358: F, t829: F) -> (F, F, F, F) {
    let t4145 = F::cast_from(0.16081979498692535067e2_f64) * t2234 * t4143;
    let t4148 = t2240 - F::cast_from(0.34246666666666666666e-1_f64) * t3300 + F::cast_from(0.5137e-1_f64) * t4106;
    let t4153 = t1358 * t1358;
    let t4154 = t4153 * t829;
    (t4145, t4148, t4153, t4154)
}
