//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1071/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1071<F: Float>(t10957: F, t10959: F, t10981: F, t11037: F, t11040: F, t11043: F, t11171: F, t11175: F, t11182: F, t11185: F, t11194: F, t2301: F, t2323: F, t271: F, t3421: F, t3440: F, t6903: F, t9068: F, t9077: F) -> (F,) {
    let t11195 = t10957 - t10959 - t11037 - t11040 - t11043 - 0.19751673498613801407e-1 * t10981 - 0.310907e-1 * t11171 * t271 + 0.2069040516770936012e4 * t6903 * t11175 - 0.23392894490538584828e1 * t9068 * t3421 + 0.34631718211362927517e2 * t9077 * t3440 + 0.35089341735807877242e1 * t2323 * t11182 - 0.23392894490538584828e1 * t2301 * t11185 + t11194;
    (t11195,)
}
