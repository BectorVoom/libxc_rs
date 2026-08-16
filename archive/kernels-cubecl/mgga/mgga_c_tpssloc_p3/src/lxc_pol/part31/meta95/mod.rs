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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta95<F: Float>(t2047: F, t218: F, t235: F, t1907: F, t226: F, t858: F, t1884: F, t259: F, t855: F, t870: F, t265: F, t394: F, t25: F, t202: F, t193: F, t504: F, t1877: F, t40: F, t28: F, dens_threshold: F, rho0: F, zeta_threshold: F, t52: F, rho1: F, t1268: F, t2036: F, t2039: F, t1992: F, t2000: F, t2004: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2048, t2051) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk580::<F>(t2047, t218, t235);
        let t2053 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk581::<F>(t1907, t2051, t226);
        let t2054 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk582::<F>(t2053, t858);
        let t2056 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk583::<F>(t1884, t2048, t2054, t259, t855);
        let t2057 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk584::<F>(t2056, t870);
        let (t2058, t2061, t2063, t2064) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk585::<F>(t265, t394, t2057, t25, t202, t2056, t193, t870);
        let (t2067, t2068, t2071) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk586::<F>(t25, t265, t504, t1877, t2058, t2064, t40, t2057, t28, t2063, dens_threshold, rho0, zeta_threshold);
        let t2075 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk587::<F>(t28, t1877, t2068, t2071, t52, t2067, dens_threshold, rho1, zeta_threshold);
        let t2079 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk588::<F>(t1268, t2036, t2039);
        let t2085 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk589::<F>(t1992, t2000, t2004);
    (t2048, t2051, t2053, t2054, t2056, t2057, t2061, t2064, t2071, t2075, t2079, t2085)
}
