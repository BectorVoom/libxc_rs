//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta94 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk611;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk612;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk613;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk614;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk615;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk616;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta94<F: Float>(t1032: F, t1276: F, t2142: F, t473: F, t265: F, t502: F, t2144: F, t2149: F, t460: F, t1300: F, t198: F, t1995: F, t336: F, t33: F, t2002: F, t57: F, t2132: F, dens_threshold: F, rho1: F, zeta_threshold: F, t2010: F, t2127: F, t118: F, t1939: F, t2036: F, t508: F, t569: F, t3: F, param_d: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t2150 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk611::<F>(t1032, t1276);
        let (t2151, t2152) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk612::<F>(t2142, t473, t2150);
        let (t2155, t2159) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk613::<F>(t265, t502, t2144, t2149, t2152, t460, t1300, t198, t1995, t336);
        let t2163 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk614::<F>(t33, t2002, t2159, t57, t2132, dens_threshold, rho1, zeta_threshold);
        let t2165 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk615::<F>(t2010, t2127);
        let (t2167, t2168, t2170) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk616::<F>(t118, t1939, t2036, t2127, t2163, t2165, t508, t569, t3, param_d);
    (t2150, t2151, t2152, t2155, t2159, t2163, t2165, t2167, t2168, t2170)
}
