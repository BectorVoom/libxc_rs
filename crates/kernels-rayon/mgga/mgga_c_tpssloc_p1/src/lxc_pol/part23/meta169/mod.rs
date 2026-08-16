//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta169 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk779;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk780;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk781;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta169(t9489: f64, t9490: f64, t9493: f64, t761: f64, t116: f64, t229: f64, t597: f64, t60: f64, t59: f64, t212: f64, t2386: f64, t131: f64, t207: f64, t2559: f64, t786: f64, t2566: f64, t2570: f64, t792: f64, t154: f64, t845: f64, t205: f64, t8705: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t9494 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk779(t9489, t9490, t9493);
        let (t9496, t9523, t9534, t9538, t9540, t9541) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk780(t761, t9494, t116, t229, t597, t60, t59, t212, t2386, t131, t207, t2559, t786);
        let (t9546, t9549, t9558, t9559, t9569) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk781(t2566, t786, t2570, t792, t154, t845, t205, t59, t8705);
    (t9494, t9496, t9523, t9534, t9538, t9540, t9541, t9546, t9549, t9558, t9559, t9569)
}
