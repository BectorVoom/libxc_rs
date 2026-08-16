//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta90 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk580;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk581;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk582;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk583;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk584;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk585;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk586;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk587;
use chunk8::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk588;
use chunk9::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk589;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta90<F: Float>(t1862: F, t67: F, t71: F, t79: F, t5: F, t1860: F, t112: F, t109: F, t107: F, t63: F, t510: F, t652: F, t193: F, t202: F, t154: F, t204: F, t209: F, t220: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1863 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk580::<F>(t1862, t67);
        let t1864 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk581::<F>(t71, t79);
        let t1865 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk582::<F>(t1863, t1864);
        let t1868 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk583::<F>(t5, t1860, t1865);
        let t1869 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk584::<F>(t112, t1868);
        let t1873 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk585::<F>(t109, t107, t63);
        let t1874 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk586::<F>(t1873, t510);
        let (t1876, t1877) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk587::<F>(t1874, t652, t193, t202);
        let t1878 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk588::<F>(t154, t204);
        let (t1879, t1880) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk589::<F>(t209, t220, t1878);
    (t1863, t1864, t1865, t1868, t1869, t1873, t1874, t1876, t1877, t1878, t1879, t1880)
}
