//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta83 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk528;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk529;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk530;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta83<F: Float>(t30: F, t1468: F, t1469: F, t1587: F, t1704: F, t265: F, t395: F, t45: F, dens_threshold: F, rho0: F, zeta_threshold: F, t1121: F, t1120: F, t128: F, t1119: F, t422: F, t1118: F) -> (F, F, F, F, F, F, F, F) {
        let (t1709, t1711) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk528::<F>(t30, t1468, t1469, t1587, t1704, t265, t395, t45, dens_threshold, rho0, zeta_threshold);
        let t1715 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk529::<F>(t1121, t1469);
        let (t1716, t1717, t1719, t1721, t1723) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk530::<F>(t1120, t1715, t128, t1119, t422, t1118);
    (t1709, t1711, t1715, t1716, t1717, t1719, t1721, t1723)
}
