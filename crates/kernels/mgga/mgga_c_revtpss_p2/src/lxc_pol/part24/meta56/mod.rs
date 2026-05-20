//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta56 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk368;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk369;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk370;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk371;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk372;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk373;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta56<F: Float>(t1408: F, t239: F, t820: F, t530: F, t549: F, t240: F, t72: F, t213: F, t555: F, t560: F, t225: F, t545: F, t869: F, t689: F, t546: F, t786: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1410 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk368::<F>(t1408, t239, t820);
        let t1412 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk369::<F>(t530, t549);
        let t1413 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk370::<F>(t1412, t240);
        let (t1414, t1424) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk371::<F>(t1413, t72, t213, t555);
        let (t1425, t1426, t1427) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk372::<F>(t560, t225);
        let (t1428, t1429, t1431, t1432) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk373::<F>(t545, t555, t869, t689, t546, t786);
    (t1410, t1412, t1413, t1414, t1424, t1425, t1426, t1427, t1428, t1429, t1431, t1432)
}
