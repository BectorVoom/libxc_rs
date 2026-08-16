//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta36 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk255;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk256;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk257;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk258;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta36<F: Float>(t145: F, t717: F, t185: F, t164: F, t159: F, t688: F, t690: F, t694: F, t699: F, t167: F, t177: F, t172: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t718, t719, t723, t724, t725, t730) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk255::<F>(t145, t717, t185, t164, t159, t688, t690, t694, t699);
        let t731 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk256::<F>(t167);
        let (t732, t738, t739) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk257::<F>(t730, t731, t177);
        let (t740, t745) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk258::<F>(t172, t739, t688, t690, t694, t699);
    (t718, t719, t723, t724, t725, t730, t731, t732, t738, t739, t740, t745)
}
