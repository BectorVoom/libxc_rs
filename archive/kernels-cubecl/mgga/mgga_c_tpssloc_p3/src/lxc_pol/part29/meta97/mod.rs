//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta97 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk634;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk635;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk636;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk637;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk638;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk639;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta97<F: Float>(t265: F, t504: F, t1238: F, t2121: F, t2124: F, t2145: F, t2155: F, t498: F, t1256: F, t193: F, t1964: F, t336: F, t28: F, t1971: F, t52: F, t2119: F, dens_threshold: F, rho1: F, zeta_threshold: F, t1979: F, t2114: F, t113: F, t1876: F, t2021: F, t510: F, t574: F, t3: F, t2028: F, t577: F, t11: F, t2: F, t584: F, t16: F, t9: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t2157, t2161) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk634::<F>(t265, t504, t1238, t2121, t2124, t2145, t2155, t498, t1256, t193, t1964, t336);
        let t2165 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk635::<F>(t28, t1971, t2161, t52, t2119, dens_threshold, rho1, zeta_threshold);
        let t2167 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk636::<F>(t1979, t2114);
        let t2169 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk637::<F>(t113, t1876, t2021, t2114, t2165, t2167, t510, t574);
        let (t2170, t2174, t2218, t2219) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk638::<F>(t2169, t3, t2028, t577, t11, t2, t584);
        let (t2220, t2221) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk639::<F>(t2219, t16, t9);
    (t2157, t2161, t2165, t2167, t2169, t2170, t2174, t2218, t2219, t2220, t2221)
}
