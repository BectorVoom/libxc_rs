//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 937/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk937<F: Float>(t12782: F, t64: F, t10205: F, t871: F, t39624: F, t39626: F, t39632: F, t39646: F, t39648: F, t39650: F, t1: F, t1415: F, t2413: F, t31730: F) -> (F, F, F, F, F, F, F, F, F) {
    let t42113 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t12782 * t64;
    let t42114 = t10205 * t871;
    let t42117 = F::cast_from(7.0_f64) / F::cast_from(512.0_f64) * t39624;
    let t42118 = F::cast_from(63.0_f64) / F::cast_from(16384.0_f64) * t39626;
    let t42119 = F::cast_from(63.0_f64) / F::cast_from(1048576.0_f64) * t39632;
    let t42120 = F::cast_from(21.0_f64) / F::cast_from(1048576.0_f64) * t39646;
    let t42121 = F::cast_from(21.0_f64) / F::cast_from(16384.0_f64) * t39648;
    let t42122 = F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t39650;
    let t42138 = t1415 * t31730 * t1 * t2413;
    (t42113, t42114, t42117, t42118, t42119, t42120, t42121, t42122, t42138)
}
