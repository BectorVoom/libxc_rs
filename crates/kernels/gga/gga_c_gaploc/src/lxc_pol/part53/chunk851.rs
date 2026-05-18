//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 851/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk851<F: Float>(t10205: F, t871: F, t39624: F, t39626: F, t39632: F, t39646: F, t39648: F, t39650: F, t40353: F, t9078: F, t986: F, t544: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t42114 = t10205 * t871;
    let t42117 = F::new(7.0) / F::new(512.0) * t39624;
    let t42118 = F::new(63.0) / F::new(16384.0) * t39626;
    let t42119 = F::new(63.0) / F::new(1048576.0) * t39632;
    let t42120 = F::new(21.0) / F::new(1048576.0) * t39646;
    let t42121 = F::new(21.0) / F::new(16384.0) * t39648;
    let t42122 = F::new(7.0) / F::new(1536.0) * t39650;
    let t42144 = F::new(0.11502877786176224903e1) * t40353;
    let t42148 = t9078 * t986;
    let t42149 = t544 * t42148;
    (t42114, t42117, t42118, t42119, t42120, t42121, t42122, t42144, t42148, t42149)
}
