//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta89 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk557;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk558;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk559;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk560;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk561;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk562;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk563;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk564;
use chunk8::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk565;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta89<F: Float>(t30: F, t33: F, t1857: F, t512: F, t1856: F, t187: F, t1344: F, t1468: F, t1348: F, t1711: F, zeta_threshold: F, t124: F, t800: F, t1319: F, t1322: F, t1334: F, t1339: F, t1342: F, t225: F, t679: F, t704: F, t1394: F, t539: F, t541: F, t543: F, t828: F, t1390: F, t1414: F, t1368: F, t1370: F, t1378: F, t1383: F, t1388: F, t1407: F, t1410: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1858, t1860, t1868) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk557::<F>(t30, t33, t1857, t512, t1856, t187, t1344, t1468, t1348, t1711, zeta_threshold);
        let t1872 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk558::<F>(t124, t1868);
        let t1873 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk559::<F>(t1872, t800);
        let (t1877, t1879) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk560::<F>(t1319, t1322, t1334, t1339, t1342, t1858, t1860, t225, t679, t704, t1394, t1868);
        let t1882 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk561::<F>(t1877, t1879, t539, t541);
        let t1883 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk562::<F>(t1882, t543);
        let t1885 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk563::<F>(t1883, t828, t1390);
        let t1889 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk564::<F>(t1414, t1868, t828);
        let t1892 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk565::<F>(t1368, t1370, t1378, t1383, t1388, t1407, t1410, t1873, t1885, t1889);
    (t1858, t1860, t1868, t1872, t1873, t1877, t1879, t1882, t1883, t1885, t1889, t1892)
}
