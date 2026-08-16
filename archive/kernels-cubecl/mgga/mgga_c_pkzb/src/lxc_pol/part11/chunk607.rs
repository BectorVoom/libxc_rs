//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 607/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk607<F: Float>(t28: F, t3315: F, t3319: F, t3330: F, t3334: F, t3347: F, t34: F, t38: F, t984: F, t991: F) -> F {
    let t3356 = F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t34 * t3315 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t34 * t3319 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t3347 * t28 - F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t991 * t984 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t38 * t3330 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t38 * t3334;
    t3356
}
