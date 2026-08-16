//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta490 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1838;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1839;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta490<F: Float>(t24826: F, t7378: F, t2147: F, t3590: F, t462: F, t7319: F, t7327: F, t7377: F, t2144: F, t3507: F, t3625: F, t1215: F, t7348: F, t1246: F, t1170: F, t7381: F, t2121: F, t210: F, t7371: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t24827, t24829, t24830, t24833) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1838::<F>(t24826, t7378, t2147, t3590, t462, t7319, t7327);
        let (t24834, t24837, t24838, t24841, t24844, t24845, t24847) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1839::<F>(t24833, t7377, t2144, t3507, t3625, t1215, t7348, t1246, t1170, t7381, t2121, t210, t7371);
    (t24827, t24829, t24830, t24833, t24834, t24837, t24838, t24841, t24844, t24845, t24847)
}
