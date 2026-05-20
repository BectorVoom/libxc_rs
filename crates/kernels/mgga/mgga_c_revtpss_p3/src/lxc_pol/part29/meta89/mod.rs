//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta89 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk535;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk536;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk537;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk538;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk539;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk540;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk541;
use chunk7::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk542;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta89<F: Float>(t33: F, t1469: F, t1587: F, t1711: F, t1837: F, t265: F, t504: F, t57: F, t1709: F, dens_threshold: F, rho1: F, zeta_threshold: F, t30: F, t1312: F, t1502: F, t1518: F, t1468: F, t513: F, t516: F, t162: F, t189: F, t512: F, t187: F, t1344: F, t1348: F, t124: F, t800: F, t1319: F, t1322: F, t1334: F, t1339: F, t1342: F, t225: F, t679: F, t704: F, t1394: F, t539: F, t541: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1843 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk535::<F>(t33, t1469, t1587, t1711, t1837, t265, t504, t57, t1709, dens_threshold, rho1, zeta_threshold);
        let (t1847, t1856) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk536::<F>(t30, t33, t1312, t1502, t1518, t1468, t513, t1711, t516, t162, zeta_threshold);
        let t1857 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk537::<F>(t1856, t189);
        let (t1858, t1860, t1868) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk538::<F>(t30, t33, t1857, t512, t1856, t187, t1344, t1468, t1348, t1711, zeta_threshold);
        let t1872 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk539::<F>(t124, t1868);
        let t1873 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk540::<F>(t1872, t800);
        let (t1877, t1879) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk541::<F>(t1319, t1322, t1334, t1339, t1342, t1858, t1860, t225, t679, t704, t1394, t1868);
        let t1882 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk542::<F>(t1877, t1879, t539, t541);
    (t1843, t1847, t1856, t1857, t1858, t1860, t1868, t1872, t1873, t1877, t1879, t1882)
}
