//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta599 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2031;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2032;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta599<F: Float>(t23041: F, t2686: F, t59: F, t9971: F, t6613: F, t9612: F, t23040: F, t2617: F, t831: F, t23061: F, t6604: F, t23099: F, t1891: F, t1895: F, t213: F, t39041: F, t1887: F, t206: F, t80845: F, t23133: F, t2703: F, t23083: F, t23089: F, t23145: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t81810, t81816, t81821, t81824, t81825, t81835, t81836) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2031::<F>(t23041, t2686, t59, t9971, t6613, t9612, t23040, t2617, t831, t23061, t6604, t23099);
        let (t81850, t81853, t81857, t81859, t81865) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2032::<F>(t1891, t1895, t213, t39041, t1887, t206, t80845, t23133, t2703, t23083, t23089, t23145, t2617);
    (t81810, t81816, t81821, t81824, t81825, t81835, t81836, t81850, t81853, t81857, t81859, t81865)
}
