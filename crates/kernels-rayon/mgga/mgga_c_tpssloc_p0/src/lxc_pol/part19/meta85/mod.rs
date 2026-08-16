//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta85 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk493;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk494;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta85(t210: f64, t214: f64, t2379: f64, t786: f64, t792: f64, t118: f64, t776: f64, t794: f64, t2553: f64, t59: f64, t835: f64, t154: f64, t116: f64, t206: f64, t212: f64, t2562: f64, t2564: f64, t2569: f64, t2571: f64, t787: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2573, t2576, t2578, t2579, t2582, t2585, t2586) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk493(t210, t214, t2379, t786, t792, t118, t776, t794, t2553, t59, t835, t154);
        let (t2588, t2591) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk494(t116, t206, t212, t2586, t2562, t2564, t2569, t2571, t2573, t2579, t2582, t787);
    (t2573, t2576, t2578, t2582, t2585, t2586, t2588, t2591)
}
