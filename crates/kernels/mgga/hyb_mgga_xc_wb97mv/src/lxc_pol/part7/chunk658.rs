//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 658/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk658<F: Float>(t7: F, t3299: F, t674: F, t1312: F, t763: F, t26: F, t1875: F, t1323: F, t222: F, t567: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t3300 = t3299 * t674;
    let t3304 = t763 * t1312;
    let t3305 = t26 * t3304;
    let t3310 = 2.0 * t1875;
    let t3311 = piecewise3(t8, 0.0, t3310);
    let t3317 = t222 * t567 * t1323;
    (t3300, t3304, t3305, t3310, t3311, t3317)
}
