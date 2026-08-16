//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 907/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk907<F: Float>(t3025: F, t7942: F, t39: F, t6023: F, t1179: F, t6025: F, t1808: F, t6033: F, t3008: F, t1897: F, t3: F, t545: F) -> (F, F, F, F, F) {
    let t7943 = t7942 * t3025;
    let t7945 = t6023 * t39;
    let t7946 = t6025 * t1179;
    let t7948 = t7945 * t7946 * t1808;
    let t7951 = t6033 * t1179;
    let t7953 = t3008 * t7951 * t1808;
    let t7956 = t1897 * t3;
    let t7958 = t3008 * t7956 * t545;
    (t7943, t7945, t7948, t7953, t7958)
}
