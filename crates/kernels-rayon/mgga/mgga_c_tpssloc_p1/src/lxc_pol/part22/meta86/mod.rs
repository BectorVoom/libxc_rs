//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta86 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk602;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk603;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk604;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk605;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk606;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk607;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta86(t1851: f64, t3: f64, t1401: f64, t1458: f64, t577: f64, t71: f64, t79: f64, t193: f64, t202: f64, t154: f64, t204: f64, t119: f64, t210: f64, t201: f64, t243: f64, t335: f64, t371: f64, t532: f64, t556: f64, t480: f64, t11: f64, t2: f64, t584: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1852, t1858, t1864, t1877) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk602(t1851, t3, t1401, t1458, t577, t71, t79, t193, t202);
        let (t1878, t1887) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk603(t154, t204, t119, t210);
        let t1891 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk604(t201, t243);
        let t1932 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk605(t335, t371);
        let t1995 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk606(t532, t556);
        let (t2130, t2218, t2219) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk607(t480, t11, t2, t584);
    (t1852, t1858, t1864, t1877, t1878, t1887, t1891, t1932, t1995, t2130, t2218, t2219)
}
