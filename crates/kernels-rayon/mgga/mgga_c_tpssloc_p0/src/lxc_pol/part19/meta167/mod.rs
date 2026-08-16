//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta167 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk795;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk796;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta167(t2566: f64, t786: f64, t2578: f64, t2570: f64, t792: f64, t118: f64, t2379: f64, t794: f64, t2553: f64, t2576: f64, t154: f64, t845: f64, t205: f64, t210: f64, t214: f64, t9458: f64, t213: f64, t776: f64, t221: f64, t59: f64, t8705: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9546, t9547, t9551, t9552, t9555, t9556, t9558) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk795(t2566, t786, t2578, t2570, t792, t118, t2379, t794, t2553, t2576, t154, t845);
        let (t9559, t9561, t9566, t9569) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk796(t205, t9558, t210, t214, t9458, t213, t776, t221, t2553, t59, t8705);
    (t9546, t9547, t9551, t9552, t9555, t9556, t9558, t9559, t9561, t9566, t9569)
}
