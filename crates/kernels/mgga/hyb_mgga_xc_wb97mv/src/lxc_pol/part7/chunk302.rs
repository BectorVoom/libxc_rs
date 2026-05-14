//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 302/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk302<F: Float>(t1014: F, t221: F, t450: F, t446: F, t437: F, t10: F, t438: F, t16: F, t566: F) -> (F, F, F, F, F, F, F, F) {
    let t1017 = 0.11073470983333333333e-2 * t221 * t1014 * t450;
    let t1018 = t446 * t446;
    let t1019 = 1.0 / t1018;
    let t1020 = t437 * t1019;
    let t1022 = 1.0 / t438 * t10;
    let t1023 = t16 * t566;
    let t1024 = t1022 * t1023;
    let t1026 = t221 * t1014;
    (t1017, t1018, t1019, t1020, t1022, t1023, t1024, t1026)
}
