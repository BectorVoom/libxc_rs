//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1122/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1122<F: Float>(t16063: F, t535: F, t517: F, t7917: F, t516: F, t1514: F, t1522: F, t1045: F, t3: F, t19: F, t1966: F, t2003: F, t1970: F, t17: F, t1867: F, t572: F) -> (F, F, F, F, F, F, F, F) {
    let t16064 = t535 * t16063;
    let t16106 = t7917 * t517;
    let t16107 = t516 * t16106;
    let t16668 = t1514 * t1522;
    let t21309 = t3 * t1045;
    let t21339 = t19 * t2003 * t1966;
    let t21342 = t19 * t2003 * t1970;
    let t21352 = t17 / t572 / t1867;
    (t16064, t16106, t16107, t16668, t21309, t21339, t21342, t21352)
}
