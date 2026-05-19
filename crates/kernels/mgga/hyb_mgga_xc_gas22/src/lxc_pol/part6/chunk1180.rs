//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1180/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1180<F: Float>(t1096: F, t1110: F, t21846: F, t7253: F, t21837: F, t2710: F, t2727: F, t441: F, t7443: F, t1055: F, t25: F, t12: F, t20626: F, t222: F, t442: F) -> (F, F, F, F) {
    let t21850 = F::cast_from(0.14035736694323150897e2_f64) * t1110 * t7253 * t21846 * t1096;
    let t21856 = F::cast_from(0.62071215503128080361e4_f64) * t441 / t2727 / t2710 * t21837 * t7443;
    let t21862 = F::new(1.0) / t25 / t1055;
    let t21864 = F::new(1.0) / t442 / t20626 * t12 * t21862 * t222 / F::new(48.0);
    (t21850, t21856, t21862, t21864)
}
