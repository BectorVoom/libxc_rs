//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1185/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1185<F: Float>(t2643: F, t7554: F, t222: F, t468: F, t7327: F, t7389: F, t7324: F, t7374: F, t1884: F, t2728: F, t2732: F, t567: F, t7448: F, t7450: F) -> (F, F, F, F, F) {
    let t21957 = t2643 * t7554;
    let t21959 = t468 * t222;
    let t21962 = F::cast_from(0.1301229756036208781e0_f64) * t21959 * t7389 * t7327;
    let t21965 = F::cast_from(0.19263893255070628431e1_f64) * t21959 * t7374 * t7324;
    let t21969 = F::cast_from(0.22911460125803964958e1_f64) * t222 * t1884 * t2728 * t2732;
    let t21973 = F::cast_from(0.68734380377411894876e1_f64) * t222 * t567 * t7448 * t7450;
    (t21957, t21962, t21965, t21969, t21973)
}
