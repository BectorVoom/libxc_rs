//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 939/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk939<F: Float>(t3720: F, t5241: F, t2679: F, t9805: F, t2610: F, t38907: F, t2033: F, t2365: F, t39040: F, t6111: F, t12251: F, t2021: F, t7372: F) -> (F, F, F, F, F) {
    let t47168 = t5241 * t3720;
    let t47170 = t9805 * t47168 * t2679;
    let t47178 = t2610 * t38907;
    let t47180 = t2033 * t2365 * t47178;
    let t47196 = t6111 * t2365 * t39040;
    let t47199 = t2021 * t12251 * t7372;
    (t47170, t47178, t47180, t47196, t47199)
}
