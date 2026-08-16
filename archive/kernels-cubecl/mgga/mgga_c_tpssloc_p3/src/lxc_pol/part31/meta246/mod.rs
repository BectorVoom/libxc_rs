//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta246 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1036;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1037;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1038;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta246<F: Float>(t1894: F, t236: F, t776: F, t6591: F, t2229: F, t61: F, t1891: F, t133: F, t119: F, t212: F, t1895: F, t213: F, t225: F, t1892: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t6593, t6594, t6597) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1036::<F>(t1894, t236, t776, t6591, t2229, t61);
        let (t6598, t6600, t6601, t6602, t6604) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1037::<F>(t1891, t6597, t133, t119, t212, t1895, t213, t225);
        let t6605 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1038::<F>(t1892, t6604);
    (t6593, t6594, t6597, t6598, t6600, t6601, t6602, t6604, t6605)
}
