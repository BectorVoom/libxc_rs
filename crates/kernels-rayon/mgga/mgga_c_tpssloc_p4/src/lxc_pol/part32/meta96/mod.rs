//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta96 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk613;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk614;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk615;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk616;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk617;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk618;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk619;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk620;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk621;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta96(t2108: f64, t67: f64, t1864: f64, t5: f64, t1860: f64, t112: f64, t265: f64, t394: f64, t1964: f64, t25: f64, t1918: f64, t40: f64, t337: f64, t50: f64, t1887: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t225: f64, t491: f64, t497: f64, t462: f64, t131: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2109 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk613(t2108, t67);
        let t2110 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk614(t1864, t2109);
        let t2113 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk615(t5, t1860, t2110);
        let t2114 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk616(t112, t2113);
        let t2116 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk617(t265, t394, t1964);
        let (t2119, t2120, t2121) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk618(t25, t1918, t2116, t40, t337, t50, t1887, dens_threshold, rho0, zeta_threshold);
        let t2122 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk619(t225, t491);
        let t2123 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk620(t2122, t497);
        let (t2124, t2127) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk621(t2123, t462, t131, t2120);
    (t2109, t2110, t2113, t2114, t2116, t2119, t2120, t2121, t2122, t2123, t2124, t2127)
}
