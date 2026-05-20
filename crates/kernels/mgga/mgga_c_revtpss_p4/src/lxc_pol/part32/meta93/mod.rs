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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta93<F: Float>(t265: F, t393: F, t2071: F, t30: F, t207: F, t2070: F, t198: F, t892: F, t502: F, t1940: F, t45: F, t33: F, dens_threshold: F, rho0: F, zeta_threshold: F, t57: F, rho1: F, t1312: F, t2052: F, t2055: F, t2016: F, t2020: F, t225: F, t561: F, t545: F, t2028: F, t2027: F, t213: F, t532: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2072, t2075, t2077, t2078) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk574::<F>(t265, t393, t2071, t30, t207, t2070, t198, t892);
        let (t2081, t2082, t2085) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk575::<F>(t30, t265, t502, t1940, t2072, t2078, t45, t2071, t33, t2077, dens_threshold, rho0, zeta_threshold);
        let t2089 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk576::<F>(t33, t1940, t2082, t2085, t57, t2081, dens_threshold, rho1, zeta_threshold);
        let t2093 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk577::<F>(t1312, t2052, t2055);
        let t2097 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk578::<F>(t2016, t2020);
        let t2098 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk579::<F>(t2097, t225);
        let (t2099, t2102) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk580::<F>(t2098, t561, t2097, t545);
        let t2103 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk581::<F>(t2028, t2102);
        let t2106 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk582::<F>(t2027, t2099, t2103, t213);
        let t2107 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk583::<F>(t2106, t532);
    (t2075, t2078, t2085, t2089, t2093, t2097, t2098, t2099, t2102, t2103, t2106, t2107)
}
