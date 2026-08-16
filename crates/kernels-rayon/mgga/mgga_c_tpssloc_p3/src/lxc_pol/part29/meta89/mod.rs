//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta89 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk582;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk583;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk584;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk585;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk586;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk587;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk588;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta89(t109: f64, t107: f64, t63: f64, t510: f64, t652: f64, t193: f64, t202: f64, t154: f64, t204: f64, t209: f64, t220: f64, t225: f64, t252: f64, t258: f64, t214: f64, t119: f64, t210: f64, t206: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1873 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk582(t109, t107, t63);
        let t1874 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk583(t1873, t510);
        let (t1876, t1877) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk584(t1874, t652, t193, t202);
        let t1878 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk585(t154, t204);
        let (t1879, t1880) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk586(t209, t220, t1878);
        let (t1882, t1883, t1884, t1887) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk587(t225, t252, t258, t214, t1880, t119, t210);
        let t1888 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk588(t1878, t1887, t206);
    (t1873, t1874, t1876, t1877, t1878, t1879, t1880, t1882, t1883, t1884, t1887, t1888)
}
