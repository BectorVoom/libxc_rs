//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta93 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk594;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk595;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk596;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk597;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk598;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk599;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk600;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta93<F: Float>(t1032: F, t1078: F, t1976: F, t359: F, t265: F, t393: F, t1978: F, t1983: F, t342: F, t1962: F, t207: F, t198: F, t892: F, t1102: F, t336: F, t30: F, t502: F, t1966: F, t45: F, t1963: F, t33: F, t1940: F, dens_threshold: F, rho0: F, zeta_threshold: F, t57: F, rho1: F, t1312: F, t1936: F, t1932: F, t196: F, t511: F, t197: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t1984 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk594::<F>(t1032, t1078);
        let (t1985, t1986) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk595::<F>(t1976, t359, t1984);
        let (t1989, t1993, t1995, t1996) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk596::<F>(t265, t393, t1978, t1983, t1986, t342, t1962, t207, t198, t892, t1102, t336);
        let (t1999, t2002, t2003) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk597::<F>(t30, t265, t502, t1966, t1996, t45, t1963, t33, t1940, t1995, dens_threshold, rho0, zeta_threshold);
        let t2007 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk598::<F>(t33, t2002, t2003, t57, t1999, dens_threshold, rho1, zeta_threshold);
        let t2011 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk599::<F>(t1312, t1936, t1932);
        let (t2013, t2014) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk600::<F>(t196, t511, t197);
    (t1984, t1985, t1986, t1989, t1993, t1996, t2003, t2007, t2011, t2013, t2014)
}
