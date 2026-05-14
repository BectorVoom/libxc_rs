//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 635/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk635<F: Float>(t43: F, t1211: F, t1226: F, t3067: F, t3068: F, t3108: F, t616: F, t635: F, t72: F, t88: F, t29: F, t125: F, t26: F, t1231: F, t668: F, t215: F, t2950: F) -> (F, F, F, F, F, F, F) {
    let t44 = 0.135e1 <= t43;
    let t3112 = piecewise3(t44, t3067, -8.0 / 3.0 * t1211 * t635 - 8.0 / 3.0 * t616 * t1226 - 8.0 / 3.0 * t3068 * t88 - 8.0 / 3.0 * t72 * t3108);
    let t3113 = t29 * t3112;
    let t3114 = t3113 * t125;
    let t3115 = t26 * t3114;
    let t3118 = t1231 * t668;
    let t3119 = t26 * t3118;
    let t3124 = t2950 * t215;
    (t3112, t3113, t3114, t3115, t3118, t3119, t3124)
}
