//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta68 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk443;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk444;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk445;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk446;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk447;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk448;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk449;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk450;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta68<F: Float>(t550: F, t73: F, t1353: F, t1392: F, t539: F, t541: F, t543: F, t828: F, t1390: F, t844: F, t247: F, t548: F, t235: F, t545: F, t239: F, t820: F, t530: F, t549: F, t240: F, t72: F, t1368: F, t1370: F, t1372: F, t1378: F, t1383: F, t1388: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1394, t1395, t1398) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk443::<F>(t550, t73, t1353, t1392, t539, t541);
        let t1399 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk444::<F>(t1398, t543);
        let (t1401, t1407, t1408) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk445::<F>(t1399, t828, t1390, t550, t844, t247, t548, t235, t545);
        let t1410 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk446::<F>(t1408, t239, t820);
        let t1412 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk447::<F>(t530, t549);
        let t1413 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk448::<F>(t1412, t240);
        let t1414 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk449::<F>(t1413, t72);
        let (t1416, t1419) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk450::<F>(t1353, t1414, t828, t1368, t1370, t1372, t1378, t1383, t1388, t1401, t1407, t1410);
    (t1394, t1395, t1398, t1399, t1401, t1408, t1410, t1412, t1413, t1414, t1416, t1419)
}
