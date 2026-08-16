//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta40 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk280;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk281;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta40<F: Float>(t287: F, t275: F, t276: F, t880: F, t273: F, t241: F, t697: F, t281: F, t283: F, t340: F, t290: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t891, t892, t893, t894, t899, t901, t904, t906, t907, t908) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk280::<F>(t287, t275, t276, t880, t273, t241, t697, t281, t283, t340);
        let t913 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk281::<F>(t290);
    (t891, t892, t893, t894, t899, t901, t904, t906, t907, t908, t913)
}
