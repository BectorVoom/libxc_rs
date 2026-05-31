//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 655/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk655<F: Float>(t1111: F, t1133: F, t1509: F, t3081: F, t3103: F, t3116: F, t3132: F, t3140: F, t4308: F, t4315: F, t4363: F, t4366: F, t5319: F, t5325: F, t5330: F, t5333: F, t5337: F) -> F {
    let t5343 = -t1111 * t5319 / F::cast_from(144.0_f64) - t3081 - t3140 - F::cast_from(0.19318136643975017455e-1_f64) * t4366 - t4308 / F::cast_from(54.0_f64) + F::cast_from(0.47333755318775392234e-1_f64) * t3116 * t5325 + F::cast_from(0.9157278480459830169e1_f64) * t3103 * t5330 - F::cast_from(0.45786392402299150845e1_f64) * t3132 * t5333 - F::cast_from(0.36221506207453157728e-2_f64) * t1133 * t5337 - F::cast_from(0.37867004255020313788e0_f64) * t4363 * t1509 + t4315 / F::cast_from(432.0_f64);
    t5343
}
