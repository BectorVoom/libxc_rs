//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 342/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk342<F: Float>(t43: F, t1193: F, t588: F, t592: F, t596: F, t600: F, t604: F, t608: F, t612: F, t1192: F) -> (F, F, F, F, F, F, F, F) {
    let t45 = 0.135e1 < t43;
    let t1196 = t588 * t1193;
    let t1198 = t592 * t1193;
    let t1200 = t596 * t1193;
    let t1202 = t600 * t1193;
    let t1204 = t604 * t1193;
    let t1206 = t608 * t1193;
    let t1208 = t612 * t1193;
    let t1211 = piecewise3(t45, 0.0, t1192);
    (t1196, t1198, t1200, t1202, t1204, t1206, t1208, t1211)
}
