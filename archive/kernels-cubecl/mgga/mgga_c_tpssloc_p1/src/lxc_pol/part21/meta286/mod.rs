//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta286 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1580;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1581;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1582;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1583;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1584;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1585;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1586;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta286<F: Float>(t10335: F, t221: F, t339: F, t2955: F, t995: F, t3069: F, t3180: F, t3036: F, t67: F, t3067: F, t3186: F, t3062: F, t820: F, t3200: F, t3051: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10383, t10385, t10388, t10390) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1580::<F>(t10335, t221, t339, t2955, t995, t3069, t3180);
        let t10401 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1581::<F>(t3036, t67);
        let t10402 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1582::<F>(t10401, t3067);
        let t10403 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1583::<F>(t10402, t3186);
        let t10408 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1584::<F>(t3062, t820);
        let t10413 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1585::<F>(t10402, t3200);
        let t10422 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1586::<F>(t3051, t820);
    (t10383, t10385, t10388, t10390, t10401, t10402, t10403, t10408, t10413, t10422)
}
