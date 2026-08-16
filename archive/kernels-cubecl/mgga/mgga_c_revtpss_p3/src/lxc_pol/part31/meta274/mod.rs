//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta274 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1230;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1231;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1232;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1233;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta274<F: Float>(t30: F, t1469: F, t1996: F, t45: F, t7794: F, t7856: F, t1544: F, t33: F, t1963: F, t1583: F, t1711: F, t1940: F, t2403: F, t7091: F, t7783: F, dens_threshold: F, rho0: F, zeta_threshold: F, t265: F, t502: F, t7855: F, t2003: F, t57: F, rho1: F, t1936: F, t4248: F, t1518: F, t93: F, t1312: F, t7741: F, t6985: F, t7725: F, t1847: F, t196: F, t197: F) -> (F, F, F, F, F, F, F, F) {
        let (t7861, t7862, t7869, t7876) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1230::<F>(t30, t1469, t1996, t45, t7794, t7856, t1544, t33, t1963, t1583, t1711, t1940, t2403, t7091, t7783, dens_threshold, rho0, zeta_threshold);
        let (t7877, t7883) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1231::<F>(t33, t265, t502, t7855, t1469, t2003, t57, t7876, t7861, dens_threshold, rho1, zeta_threshold);
        let (t7888, t7889) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1232::<F>(t1936, t4248, t1518, t93);
        let (t7894, t7897, t7898) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1233::<F>(t1936, t7889, t1312, t7741, t1518, t6985, t7725, t7888, t1847, t196, t197);
    (t7862, t7869, t7877, t7883, t7889, t7894, t7897, t7898)
}
