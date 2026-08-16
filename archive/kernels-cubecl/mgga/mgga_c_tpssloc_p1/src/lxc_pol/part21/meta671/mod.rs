//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta671 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2474;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2475;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta671<F: Float>(t1227: F, t248: F, t3243: F, t45046: F, t221: F, t44483: F, t456: F, t3575: F, t42386: F, t11888: F, t11914: F, t11784: F, t820: F, t11779: F, t11677: F, t11907: F, t11904: F, t1174: F, t3556: F, t698: F, t11153: F, t1176: F, t11881: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t45049, t45112, t45113, t45114, t45119, t45124) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2474::<F>(t1227, t248, t3243, t45046, t221, t44483, t456, t3575, t42386, t11888, t11914, t11784, t820);
        let (t45128, t45134, t45162, t45178, t45192, t45197) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2475::<F>(t11779, t820, t11677, t11907, t11904, t1174, t3556, t698, t11153, t1176, t11881, t45113);
    (t45049, t45112, t45114, t45119, t45124, t45128, t45134, t45162, t45178, t45192, t45197)
}
