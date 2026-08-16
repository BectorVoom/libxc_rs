//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta91 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk584;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk585;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk586;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk587;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk588;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk589;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta91(t209: f64, t220: f64, t1878: f64, t225: f64, t252: f64, t258: f64, t214: f64, t119: f64, t210: f64, t206: f64, t201: f64, t243: f64, t598: f64, t213: f64, t234: f64, t236: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1879, t1880) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk584(t209, t220, t1878);
        let (t1882, t1883, t1884, t1887) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk585(t225, t252, t258, t214, t1880, t119, t210);
        let t1888 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk586(t1878, t1887, t206);
        let t1891 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk587(t201, t243);
        let (t1892, t1893, t1894) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk588(t1891, t598, t213, t225, t234);
        let t1895 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk589(t1894, t236);
    (t1879, t1880, t1882, t1883, t1884, t1887, t1888, t1891, t1892, t1893, t1894, t1895)
}
