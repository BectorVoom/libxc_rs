//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta46 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk308;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk309;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk310;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk311;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk312;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk313;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta46<F: Float>(t893: F, t914: F, t880: F, t886: F, t307: F, t302: F, t906: F, t897: F, t902: F, t910: F, t310: F, t324: F, t320: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t916, t919) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk308::<F>(t893, t914, t880, t886);
        let (t922, t923, t924, t931) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk309::<F>(t307, t302, t880, t906, t886, t897, t902, t910);
        let t932 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk310::<F>(t310);
        let t933 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk311::<F>(t931, t932);
        let t938 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk312::<F>(t880, t886);
        let (t939, t941, t942) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk313::<F>(t324, t938, t320);
    (t916, t919, t922, t923, t924, t931, t932, t933, t938, t939, t941, t942)
}
