//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1383/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1383<F: Float>(t8973: F, t9104: F, t2515: F, t4273: F, t7075: F, t11038: F, t21537: F, t2479: F, t2478: F, t4270: F, t11031: F, t2521: F) -> (F, F, F, F, F, F) {
    let t30009 = F::new(12.0) * t9104 * t8973;
    let t30012 = F::cast_from(0.96491876992155210402e2_f64) * t7075 * t4273 * t2515;
    let t30015 = F::cast_from(0.62071215503128080361e4_f64) * t21537 * t11038 * t2479;
    let t30018 = F::new(2.0) * t2478 * t4270 * t2515;
    let t30021 = F::cast_from(0.96491876992155210402e2_f64) * t7075 * t11031 * t2479;
    let t30024 = F::cast_from(0.16081979498692535067e2_f64) * t2521 * t11031 * t2515;
    (t30009, t30012, t30015, t30018, t30021, t30024)
}
