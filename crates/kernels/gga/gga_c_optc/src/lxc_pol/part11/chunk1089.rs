//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1089/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1089<F: Float>(t1179: F, t54304: F, t17987: F, t3244: F, t9142: F, t1150: F, t17993: F, t2367: F, t12799: F, t16007: F, t1129: F, t18092: F, t3107: F, t54753: F, t3119: F, t5096: F) -> (F, F, F, F, F, F, F) {
    let t55645 = t1179 * t54304;
    let t55734 = t3244 * t9142 * t17987;
    let t55749 = t1150 * t2367 * t17993;
    let t55751 = t12799 * t16007;
    let t55753 = t18092 * t1129;
    let t55761 = t54753 * t3107;
    let t55768 = t3119 * t5096;
    (t55645, t55734, t55749, t55751, t55753, t55761, t55768)
}
