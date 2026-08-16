//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta90 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk630;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk631;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk632;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk633;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk634;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk635;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta90(t2244: f64, t2274: f64, t2250: f64, t55: f64, t240: f64, t59: f64, t2262: f64, t2268: f64, t2271: f64, t39: f64, t44: f64, t51: f64, t615: f64, t618: f64, t33: f64, t40: f64, t632: f64, t73: f64, t52: f64, t636: f64, t76: f64, t634: f64, t638: f64, t72: f64, t2245: f64, t2252: f64, t2255: f64, t609: f64, t629: f64, t642: f64, t66: f64, t80: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2275, t2278, t2281) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk630(t2244, t2274, t2250, t55, t240, t59);
        let (t2282, t2283, t2284) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk631(t2281, t2262, t2268, t2271, t2275, t2278, t39, t44, t51, t615, t618, t33);
        let (t2289, t2291) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk632(t40, t632, t73);
        let (t2296, t2298) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk633(t52, t636, t76);
        let t2304 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk634(t2244, t2250, t2291, t2298, t634, t638, t72);
        let t2307 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk635(t2245, t2252, t2255, t2284, t2304, t609, t629, t642, t66, t80);
    (t2275, t2278, t2281, t2282, t2283, t2284, t2289, t2291, t2296, t2298, t2304, t2307)
}
