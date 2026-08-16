//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 863/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk863<F: Float>(t12782: F, t64: F, t10205: F, t871: F, t2748: F, t3113: F, t39624: F, t39626: F, t39632: F, t39646: F, t39648: F, t39650: F) -> (F, F, F, F, F, F, F, F, F) {
    let t42113 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t12782 * t64;
    let t42114 = t10205 * t871;
    let t42115 = t2748 * t3113;
    let t42117 = F::cast_from(7.0_f64) / F::cast_from(512.0_f64) * t39624;
    let t42118 = F::cast_from(63.0_f64) / F::cast_from(16384.0_f64) * t39626;
    let t42119 = F::cast_from(63.0_f64) / F::cast_from(1048576.0_f64) * t39632;
    let t42120 = F::cast_from(21.0_f64) / F::cast_from(1048576.0_f64) * t39646;
    let t42121 = F::cast_from(21.0_f64) / F::cast_from(16384.0_f64) * t39648;
    let t42122 = F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t39650;
    (t42113, t42114, t42115, t42117, t42118, t42119, t42120, t42121, t42122)
}
