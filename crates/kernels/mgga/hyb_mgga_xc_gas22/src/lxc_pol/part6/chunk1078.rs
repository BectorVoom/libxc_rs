//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1078/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1078<F: Float>(t3984: F, t764: F, t26: F, t10: F, t10325: F, t18: F, t1291: F, t2950: F, t4088: F, t550: F, t136: F, t1240: F, t3157: F) -> (F, F, F, F, F, F, F) {
    let t10445 = t764 * t3984;
    let t10446 = t26 * t10445;
    let t10450 = t10325 * t10 * t18;
    let t10457 = t2950 * t1291;
    let t10460 = t550 * t4088;
    let t10461 = t136 * t10460;
    let t10463 = t1240 * t3157;
    (t10445, t10446, t10450, t10457, t10460, t10461, t10463)
}
