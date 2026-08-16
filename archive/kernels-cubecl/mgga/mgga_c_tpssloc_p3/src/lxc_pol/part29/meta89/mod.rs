//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta89 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk582;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk583;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk584;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk585;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk586;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk587;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk588;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta89<F: Float>(t109: F, t107: F, t63: F, t510: F, t652: F, t193: F, t202: F, t154: F, t204: F, t209: F, t220: F, t225: F, t252: F, t258: F, t214: F, t119: F, t210: F, t206: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1873 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk582::<F>(t109, t107, t63);
        let t1874 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk583::<F>(t1873, t510);
        let (t1876, t1877) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk584::<F>(t1874, t652, t193, t202);
        let t1878 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk585::<F>(t154, t204);
        let (t1879, t1880) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk586::<F>(t209, t220, t1878);
        let (t1882, t1883, t1884, t1887) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk587::<F>(t225, t252, t258, t214, t1880, t119, t210);
        let t1888 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk588::<F>(t1878, t1887, t206);
    (t1873, t1874, t1876, t1877, t1878, t1879, t1880, t1882, t1883, t1884, t1887, t1888)
}
