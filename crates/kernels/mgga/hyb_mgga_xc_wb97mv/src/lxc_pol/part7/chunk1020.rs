//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1020/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1020<F: Float>(t10003: F, t10059: F, t10128: F, t10197: F, t9771: F, t9819: F, t9886: F, t9944: F, t496: F, t125: F, t3854: F, t544: F, t1224: F, t2988: F, t3864: F) -> (F, F, F, F, F, F, F) {
    let t10200 = t9771 + t9819 + t9886 + t9944 + t10003 + t10059 + t10128 + t10197;
    let t10201 = t496 * t10200;
    let t10214 = t125 * t3854;
    let t10215 = t10214 * t544;
    let t10219 = t2988 * t1224;
    let t10223 = t125 * t3864;
    let t10224 = t10223 * t544;
    (t10200, t10201, t10214, t10215, t10219, t10223, t10224)
}
