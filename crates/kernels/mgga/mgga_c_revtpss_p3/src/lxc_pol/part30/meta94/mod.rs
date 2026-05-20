//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta94 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk599;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk600;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk601;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk602;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk603;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk604;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta94<F: Float>(t2042: F, t572: F, t55: F, t61: F, t68: F, t72: F, t1927: F, t5: F, t1923: F, t117: F, t265: F, t393: F, t1995: F, t30: F, t1966: F, t45: F, t343: F, t136: F, t473: F, t479: F, dens_threshold: F, rho0: F, sigma2: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2044, t2121, t2122) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk599::<F>(t2042, t572, t55, t61, t68, t72);
        let t2123 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk600::<F>(t1927, t2122);
        let t2126 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk601::<F>(t5, t1923, t2123);
        let t2127 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk602::<F>(t117, t2126);
        let t2129 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk603::<F>(t265, t393, t1995);
        let (t2132, t2133, t2134, t2137, t2138) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk604::<F>(t30, t1966, t2129, t45, t343, t55, t136, t473, t479, dens_threshold, rho0, sigma2, zeta_threshold);
    (t2044, t2121, t2122, t2123, t2126, t2127, t2129, t2132, t2133, t2134, t2137, t2138)
}
