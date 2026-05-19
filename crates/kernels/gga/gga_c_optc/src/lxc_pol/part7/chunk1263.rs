//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1263/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1263<F: Float>(t2822: F, t7214: F, t7325: F, t10849: F, t8272: F, t10845: F, t7179: F, t10109: F, t2433: F, t24704: F, t24708: F, t24712: F, t24715: F, t24718: F, t2563: F, t2569: F, t277: F, t7263: F, t8273: F, t95: F) -> F {
    let t26073 = t2822 * t2822;
    let t26080 = t7325 * t7214;
    let t26084 = t10849 * t8272;
    let t26087 = t10845 * t7179;
    let t26090 = -F::cast_from(0.77534644304710291488e-2_f64) * t95 * t277 * t26073 * t2569 - F::new(2.0) * t7263 * t2563 - F::new(200.0) / F::new(9.0) * t26080 + F::new(8.0) / F::new(3.0) * t10109 * t8273 + F::new(800.0) / F::new(81.0) * t2433 * t26084 - F::new(400.0) / F::new(27.0) * t2433 * t26087 + t24704 + t24708 + t24712 - t24715 - t24718;
    t26090
}
