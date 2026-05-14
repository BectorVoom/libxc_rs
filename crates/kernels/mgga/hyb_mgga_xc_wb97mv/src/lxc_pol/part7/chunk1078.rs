//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1078/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1078<F: Float>(t11317: F, t238: F, t242: F, t11297: F, t11299: F, t11304: F, t11308: F, t11311: F, t11315: F, t7294: F, t7391: F, t9360: F, t9423: F, t9424: F, t11295: F, t957: F) -> (F, F, F) {
    let t11319 = t238 * t242 * t11317;
    let t11321 = 0.15358125e0 * t11297 + 0.3071625e0 * t11299 - t7391 + 0.27385555555555555556e0 * t7294 + 0.5477111111111111111e0 * t9360 - t9423 - t9424 - 0.16431333333333333333e0 * t11304 + 0.49294e0 * t11308 - 0.16431333333333333333e0 * t11311 + 0.24647e0 * t11315 + 0.24647e0 * t11319;
    let t11322 = t11295 + t11321;
    let t11323 = t11322 * t957;
    (t11319, t11322, t11323)
}
