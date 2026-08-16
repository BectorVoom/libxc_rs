//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta93 (260520-c91 hierarchical CSE).
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
mod chunk9;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk574;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk575;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk576;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk577;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk578;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk579;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk580;
use chunk7::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk581;
use chunk8::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk582;
use chunk9::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk583;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta93(t265: f64, t393: f64, t2071: f64, t30: f64, t207: f64, t2070: f64, t198: f64, t892: f64, t502: f64, t1940: f64, t45: f64, t33: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t57: f64, rho1: f64, t1312: f64, t2052: f64, t2055: f64, t2016: f64, t2020: f64, t225: f64, t561: f64, t545: f64, t2028: f64, t2027: f64, t213: f64, t532: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2072, t2075, t2077, t2078) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk574(t265, t393, t2071, t30, t207, t2070, t198, t892);
        let (t2081, t2082, t2085) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk575(t30, t265, t502, t1940, t2072, t2078, t45, t2071, t33, t2077, dens_threshold, rho0, zeta_threshold);
        let t2089 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk576(t33, t1940, t2082, t2085, t57, t2081, dens_threshold, rho1, zeta_threshold);
        let t2093 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk577(t1312, t2052, t2055);
        let t2097 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk578(t2016, t2020);
        let t2098 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk579(t2097, t225);
        let (t2099, t2102) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk580(t2098, t561, t2097, t545);
        let t2103 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk581(t2028, t2102);
        let t2106 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk582(t2027, t2099, t2103, t213);
        let t2107 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk583(t2106, t532);
    (t2075, t2078, t2085, t2089, t2093, t2097, t2098, t2099, t2102, t2103, t2106, t2107)
}
