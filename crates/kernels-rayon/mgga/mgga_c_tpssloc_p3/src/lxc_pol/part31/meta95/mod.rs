//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta95 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk580;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk581;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk582;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk583;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk584;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk585;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk586;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk587;
use chunk8::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk588;
use chunk9::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk589;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta95(t2047: f64, t218: f64, t235: f64, t1907: f64, t226: f64, t858: f64, t1884: f64, t259: f64, t855: f64, t870: f64, t265: f64, t394: f64, t25: f64, t202: f64, t193: f64, t504: f64, t1877: f64, t40: f64, t28: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t52: f64, rho1: f64, t1268: f64, t2036: f64, t2039: f64, t1992: f64, t2000: f64, t2004: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2048, t2051) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk580(t2047, t218, t235);
        let t2053 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk581(t1907, t2051, t226);
        let t2054 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk582(t2053, t858);
        let t2056 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk583(t1884, t2048, t2054, t259, t855);
        let t2057 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk584(t2056, t870);
        let (t2058, t2061, t2063, t2064) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk585(t265, t394, t2057, t25, t202, t2056, t193, t870);
        let (t2067, t2068, t2071) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk586(t25, t265, t504, t1877, t2058, t2064, t40, t2057, t28, t2063, dens_threshold, rho0, zeta_threshold);
        let t2075 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk587(t28, t1877, t2068, t2071, t52, t2067, dens_threshold, rho1, zeta_threshold);
        let t2079 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk588(t1268, t2036, t2039);
        let t2085 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk589(t1992, t2000, t2004);
    (t2048, t2051, t2053, t2054, t2056, t2057, t2061, t2064, t2071, t2075, t2079, t2085)
}
