//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta276 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1447;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1448;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1449;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1450;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1451;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta276<F: Float>(t1017: F, t3087: F, t1015: F, t1012: F, t2940: F, t2952: F, t2928: F, t320: F, t2906: F, t950: F, t2932: F, t959: F, t10195: F, t2768: F, t123: F, t10250: F, t882: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10516, t10517) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1447::<F>(t1017, t3087, t1015, t1012);
        let (t10521, t10523) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1448::<F>(t2940, t2952, t2928, t320);
        let t10524 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1449::<F>(t2906, t950);
        let (t10526, t10528, t10529, t10530) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1450::<F>(t10523, t10524, t2932, t959, t10195, t2768, t123);
        let (t10537, t10538) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1451::<F>(t10250, t882, t123);
    (t10516, t10517, t10521, t10523, t10524, t10526, t10528, t10529, t10530, t10537, t10538)
}
