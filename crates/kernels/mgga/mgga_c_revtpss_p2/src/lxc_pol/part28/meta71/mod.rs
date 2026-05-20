//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta71 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk459;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk460;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk461;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk462;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk463;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk464;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta71<F: Float>(t546: F, t550: F, t808: F, t807: F, t547: F, t786: F, t814: F, t816: F, t544: F, t235: F, t239: F, t820: F, t240: F, t72: F, t1319: F, t1322: F, t1332: F, t1334: F, t1336: F, t1339: F, t1342: F, t225: F, t679: F, t704: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1376, t1378, t1379, t1381, t1383, t1384, t1385) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk459::<F>(t546, t550, t808, t807, t547, t786, t814, t816, t544);
        let t1386 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk460::<F>(t1385, t235);
        let t1388 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk461::<F>(t1386, t239, t820);
        let t1389 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk462::<F>(t240, t550);
        let t1390 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk463::<F>(t1389, t72);
        let t1392 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk464::<F>(t1319, t1322, t1332, t1334, t1336, t1339, t1342, t225, t679, t704);
    (t1376, t1378, t1379, t1381, t1383, t1384, t1385, t1386, t1388, t1389, t1390, t1392)
}
