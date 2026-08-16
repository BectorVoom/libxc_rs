//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta320 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1586;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1587;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1588;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1589;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1590;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta320<F: Float>(t11579: F, t3449: F, t3247: F, t460: F, t2244: F, t1176: F, t134: F, t1184: F, t3451: F, t3447: F, t3448: F, t3475: F, t11549: F, t11556: F, t11558: F, t11561: F, t11563: F, t11566: F, t11572: F, t11576: F, t1174: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11580, t11583) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1586::<F>(t11579, t3449, t3247, t460);
        let t11584 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1587::<F>(t11583, t2244);
        let (t11585, t11588) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1588::<F>(t11584, t3449, t1176, t134);
        let t11589 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1589::<F>(t11588, t1184);
        let (t11590, t11591, t11593, t11594, t11597) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1590::<F>(t11589, t3451, t3447, t3448, t3475, t11549, t11556, t11558, t11561, t11563, t11566, t11572, t11576, t11580, t11585, t1174);
    (t11580, t11583, t11584, t11585, t11588, t11589, t11590, t11591, t11593, t11594, t11597)
}
