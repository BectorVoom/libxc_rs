//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta232 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1324;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1325;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta232(t789: f64, t9541: f64, t2563: f64, t2582: f64, t2566: f64, t786: f64, t2578: f64, t2570: f64, t792: f64, t118: f64, t2379: f64, t794: f64, t2553: f64, t2576: f64, t154: f64, t845: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9542, t9544, t9546, t9547, t9549, t9551) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1324(t789, t9541, t2563, t2582, t2566, t786, t2578, t2570, t792, t118, t2379, t794);
        let (t9552, t9555, t9556, t9558) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1325(t9549, t9551, t118, t2553, t794, t2576, t154, t845);
    (t9542, t9544, t9546, t9547, t9551, t9552, t9555, t9556, t9558)
}
