//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta52 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk349;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk350;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk351;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk352;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk353;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk354;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk355;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta52<F: Float>(t493: F, t225: F, t473: F, t487: F, t1032: F, t1243: F, t460: F, t355: F, t471: F, t498: F, t116: F, t93: F, t22: F, t583: F, t521: F, t19: F, t588: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1275, t1276, t1277) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk349::<F>(t493, t225);
        let t1280 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk350::<F>(t473, t487);
        let t1284 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk351::<F>(t1032, t1243);
        let t1285 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk352::<F>(t1284, t460);
        let t1287 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk353::<F>(t355, t471);
        let (t1300, t1312, t1317) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk354::<F>(t498, t116, t93, t22, t583);
        let (t1319, t1320) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk355::<F>(t1317, t521, t19, t588);
    (t1275, t1276, t1277, t1280, t1284, t1285, t1287, t1300, t1312, t1317, t1319, t1320)
}
