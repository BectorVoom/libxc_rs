//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1219/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1219<F: Float>(t1781: F, t862: F, t866: F, t2548: F, t7256: F, t22015: F, t2626: F, t7410: F, t2623: F, t7402: F, t7298: F, t864: F) -> (F, F, F, F, F) {
    let t25172 = t862 * t1781 * t866;
    let t25174 = t2548 * t7256;
    let t25175 = t25174 * t22015;
    let t25179 = t7410 * t2626;
    let t25181 = t2623 * t7402;
    let t25183 = t864 * t7298;
    (t25172, t25175, t25179, t25181, t25183)
}
