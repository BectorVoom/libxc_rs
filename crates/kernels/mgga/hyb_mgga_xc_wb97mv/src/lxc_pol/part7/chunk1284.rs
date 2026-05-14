//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1284/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1284<F: Float>(t11224: F, t1373: F, t2322: F, t2325: F, t2345: F, t26476: F, t30801: F, t30973: F, t30975: F, t30977: F, t30979: F, t30992: F, t30995: F, t30998: F, t31132: F, t31135: F, t31138: F, t855: F) -> (F,) {
    let t31494 = t30973 + t30975 + t30977 + t30979 - 0.34631718211362927518e2 * t855 * t2322 * t30801 * t2325 - t30992 - t30995 + t30998 - 0.17315859105681463759e2 * t11224 * t2345 - 0.11696447245269292414e1 * t26476 * t1373 - t31132 - t31135 - t31138;
    (t31494,)
}
