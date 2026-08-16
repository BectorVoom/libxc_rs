//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta98 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk629;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk630;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk631;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk632;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk633;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk634;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk635;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk636;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk637;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta98(t2144: f64, t466: f64, t225: f64, t477: f64, t491: f64, t462: f64, t493: f64, t2121: f64, t470: f64, t1241: f64, t265: f64, t504: f64, t1238: f64, t2124: f64, t498: f64, t1256: f64, t193: f64, t1964: f64, t336: f64, t28: f64, t1971: f64, t52: f64, t2119: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t1979: f64, t2114: f64, t113: f64, t1876: f64, t2021: f64, t510: f64, t574: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2145, t2147) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk629(t2144, t466, t225, t477);
        let t2148 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk630(t2147, t491);
        let (t2149, t2152) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk631(t2148, t462, t2144, t493);
        let t2154 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk632(t2121, t2149, t2152, t470);
        let t2155 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk633(t1241, t2154);
        let (t2157, t2161) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk634(t265, t504, t1238, t2121, t2124, t2145, t2155, t498, t1256, t193, t1964, t336);
        let t2165 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk635(t28, t1971, t2161, t52, t2119, dens_threshold, rho1, zeta_threshold);
        let t2167 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk636(t1979, t2114);
        let t2169 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk637(t113, t1876, t2021, t2114, t2165, t2167, t510, t574);
    (t2145, t2147, t2148, t2149, t2152, t2154, t2155, t2157, t2161, t2165, t2167, t2169)
}
