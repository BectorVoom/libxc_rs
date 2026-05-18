//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 956/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk956<F: Float>(t3133: F, t9129: F, t1115: F, t7274: F, t1162: F, t3128: F, t8469: F, t3245: F, t9040: F, t1113: F, t24: F) -> (F, F, F, F, F) {
    let t9130 = t9129 * t3133;
    let t9133 = t7274 * t1115;
    let t9134 = t1162 * t9133;
    let t9136 = t8469 * t3128;
    let t9139 = t3245 * t9040;
    let t9142 = t24 * t1113;
    (t9130, t9134, t9136, t9139, t9142)
}
