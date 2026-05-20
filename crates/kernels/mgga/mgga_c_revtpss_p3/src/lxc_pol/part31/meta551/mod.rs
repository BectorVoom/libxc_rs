//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta551 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1951;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1952;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1953;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta551<F: Float>(t30: F, t1469: F, t1996: F, t29726: F, t29931: F, t45: F, t5825: F, t7856: F, t33: F, t5966: F, t1963: F, t25759: F, t29598: F, dens_threshold: F, rho0: F, zeta_threshold: F, t1544: F, t1711: F, t5962: F, t6079: F, t1583: F, t6075: F, t1940: F, t2403: F, t25206: F, t25445: F, t27368: F, t29705: F, t4541: F, t6416: F, t7091: F, t7783: F, t7862: F, t7869: F, t265: F, t502: F, t29930: F, t2003: F, t57: F, t7877: F, t118: F, t1502: F, t1843: F, t1932: F, t2007: F, t29497: F, t29501: F, t29504: F, t29507: F, t29510: F, t29512: F, t29569: F, t29573: F, t29578: F, t29580: F, t29582: F, t29585: F, t29590: F, t508: F, t5877: F, t5884: F, t6765: F, t7725: F, t7883: F, rho1: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t29938, t29939, t29940, t29946) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1951::<F>(t30, t1469, t1996, t29726, t29931, t45, t5825, t7856, t33, t5966, t1963, t25759, t29598, dens_threshold, rho0, zeta_threshold);
        let (t29949, t29953, t29964, t29967, t29970, t29977) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1952::<F>(t1544, t1711, t33, t5962, t6079, t1583, t6075, t1940, t1963, t2403, t25206, t25445, t27368, t29705, t29940, t29946, t4541, t6416, t7091, t7783, t7862, t7869);
        let (t29978, t29986, t29991) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1953::<F>(t33, t265, t502, t29930, t1469, t2003, t29977, t57, t5825, t7877, t29938, t118, t1502, t1843, t1932, t2007, t29497, t29501, t29504, t29507, t29510, t29512, t29569, t29573, t29578, t29580, t29582, t29585, t29590, t508, t5877, t5884, t6765, t7725, t7883, dens_threshold, rho1, zeta_threshold);
    (t29939, t29940, t29946, t29949, t29953, t29964, t29967, t29970, t29978, t29986, t29991)
}
