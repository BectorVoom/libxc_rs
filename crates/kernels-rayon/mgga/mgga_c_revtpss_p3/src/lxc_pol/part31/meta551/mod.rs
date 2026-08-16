//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta551 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1951;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1952;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1953;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta551(t30: f64, t1469: f64, t1996: f64, t29726: f64, t29931: f64, t45: f64, t5825: f64, t7856: f64, t33: f64, t5966: f64, t1963: f64, t25759: f64, t29598: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t1544: f64, t1711: f64, t5962: f64, t6079: f64, t1583: f64, t6075: f64, t1940: f64, t2403: f64, t25206: f64, t25445: f64, t27368: f64, t29705: f64, t4541: f64, t6416: f64, t7091: f64, t7783: f64, t7862: f64, t7869: f64, t265: f64, t502: f64, t29930: f64, t2003: f64, t57: f64, t7877: f64, t118: f64, t1502: f64, t1843: f64, t1932: f64, t2007: f64, t29497: f64, t29501: f64, t29504: f64, t29507: f64, t29510: f64, t29512: f64, t29569: f64, t29573: f64, t29578: f64, t29580: f64, t29582: f64, t29585: f64, t29590: f64, t508: f64, t5877: f64, t5884: f64, t6765: f64, t7725: f64, t7883: f64, rho1: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29938, t29939, t29940, t29946) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1951(t30, t1469, t1996, t29726, t29931, t45, t5825, t7856, t33, t5966, t1963, t25759, t29598, dens_threshold, rho0, zeta_threshold);
        let (t29949, t29953, t29964, t29967, t29970, t29977) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1952(t1544, t1711, t33, t5962, t6079, t1583, t6075, t1940, t1963, t2403, t25206, t25445, t27368, t29705, t29940, t29946, t4541, t6416, t7091, t7783, t7862, t7869);
        let (t29978, t29986, t29991) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1953(t33, t265, t502, t29930, t1469, t2003, t29977, t57, t5825, t7877, t29938, t118, t1502, t1843, t1932, t2007, t29497, t29501, t29504, t29507, t29510, t29512, t29569, t29573, t29578, t29580, t29582, t29585, t29590, t508, t5877, t5884, t6765, t7725, t7883, dens_threshold, rho1, zeta_threshold);
    (t29939, t29940, t29946, t29949, t29953, t29964, t29967, t29970, t29978, t29986, t29991)
}
