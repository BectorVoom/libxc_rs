//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta55 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk362;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk363;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk364;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk365;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk366;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk367;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta55<F: Float>(t1376: F, t807: F, t547: F, t786: F, t550: F, t814: F, t816: F, t544: F, t235: F, t239: F, t820: F, t240: F, t72: F, t73: F, t844: F, t247: F, t548: F, t545: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1378, t1379, t1383, t1384, t1385) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk362::<F>(t1376, t807, t547, t786, t550, t814, t816, t544);
        let t1386 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk363::<F>(t1385, t235);
        let t1388 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk364::<F>(t1386, t239, t820);
        let t1389 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk365::<F>(t240, t550);
        let t1390 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk366::<F>(t1389, t72);
        let (t1394, t1407, t1408) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk367::<F>(t550, t73, t844, t247, t548, t235, t545);
    (t1378, t1379, t1383, t1384, t1385, t1386, t1388, t1389, t1390, t1394, t1407, t1408)
}
