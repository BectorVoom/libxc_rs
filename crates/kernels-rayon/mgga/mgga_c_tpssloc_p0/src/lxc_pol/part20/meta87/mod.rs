//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta87 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk610;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk611;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk612;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk613;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk614;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk615;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk616;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta87(t154: f64, t204: f64, t119: f64, t210: f64, t201: f64, t243: f64, t365: f64, t335: f64, t371: f64, t532: f64, t556: f64, t11: f64, t2: f64, t584: f64, t16: f64, t9: f64, t587: f64, t591: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1878, t1887) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk610(t154, t204, t119, t210);
        let t1891 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk611(t201, t243);
        let (t1929, t1932) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk612(t365, t335, t371);
        let t1995 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk613(t532, t556);
        let (t2218, t2219) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk614(t11, t2, t584);
        let (t2220, t2221) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk615(t2219, t16, t9);
        let (t2222, t2223) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk616(t2221, t587, t591);
    (t1878, t1887, t1891, t1929, t1932, t1995, t2218, t2219, t2220, t2221, t2222, t2223)
}
