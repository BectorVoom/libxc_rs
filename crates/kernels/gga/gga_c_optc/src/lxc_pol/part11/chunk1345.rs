//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1345/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1345<F: Float>(t1027: F, t55927: F, t3018: F, t5171: F, t5186: F, t15562: F, t5308: F, t5202: F) -> (F, F, F, F) {
    let t58295 = t1027 * t55927;
    let t58308 = F::new(36.0) * t3018 * t5171 * t5186;
    let t58310 = F::new(0.1038945353962551798e3) * t15562 * t5308;
    let t58311 = t5202 * t5202;
    (t58295, t58308, t58310, t58311)
}
