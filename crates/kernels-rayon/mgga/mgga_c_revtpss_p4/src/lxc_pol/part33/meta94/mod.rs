//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta94 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk611;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk612;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk613;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk614;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk615;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk616;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta94(t1032: f64, t1276: f64, t2142: f64, t473: f64, t265: f64, t502: f64, t2144: f64, t2149: f64, t460: f64, t1300: f64, t198: f64, t1995: f64, t336: f64, t33: f64, t2002: f64, t57: f64, t2132: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t2010: f64, t2127: f64, t118: f64, t1939: f64, t2036: f64, t508: f64, t569: f64, t3: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2150 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk611(t1032, t1276);
        let (t2151, t2152) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk612(t2142, t473, t2150);
        let (t2155, t2159) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk613(t265, t502, t2144, t2149, t2152, t460, t1300, t198, t1995, t336);
        let t2163 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk614(t33, t2002, t2159, t57, t2132, dens_threshold, rho1, zeta_threshold);
        let t2165 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk615(t2010, t2127);
        let (t2167, t2168, t2170) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk616(t118, t1939, t2036, t2127, t2163, t2165, t508, t569, t3, param_d);
    (t2150, t2151, t2152, t2155, t2159, t2163, t2165, t2167, t2168, t2170)
}
