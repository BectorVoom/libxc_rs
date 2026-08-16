//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta320 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1586;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1587;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1588;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1589;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1590;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta320(t11579: f64, t3449: f64, t3247: f64, t460: f64, t2244: f64, t1176: f64, t134: f64, t1184: f64, t3451: f64, t3447: f64, t3448: f64, t3475: f64, t11549: f64, t11556: f64, t11558: f64, t11561: f64, t11563: f64, t11566: f64, t11572: f64, t11576: f64, t1174: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11580, t11583) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1586(t11579, t3449, t3247, t460);
        let t11584 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1587(t11583, t2244);
        let (t11585, t11588) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1588(t11584, t3449, t1176, t134);
        let t11589 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1589(t11588, t1184);
        let (t11590, t11591, t11593, t11594, t11597) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1590(t11589, t3451, t3447, t3448, t3475, t11549, t11556, t11558, t11561, t11563, t11566, t11572, t11576, t11580, t11585, t1174);
    (t11580, t11583, t11584, t11585, t11588, t11589, t11590, t11591, t11593, t11594, t11597)
}
