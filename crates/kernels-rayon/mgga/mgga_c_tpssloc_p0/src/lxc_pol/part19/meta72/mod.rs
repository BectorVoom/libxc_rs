//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta72 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk441;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk442;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk443;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta72(t40: f64, t632: f64, t73: f64, t52: f64, t636: f64, t76: f64, t2244: f64, t2250: f64, t634: f64, t638: f64, t72: f64, t2245: f64, t2252: f64, t2255: f64, t2284: f64, t609: f64, t629: f64, t642: f64, t66: f64, t80: f64, t5: f64, t2233: f64, t2235: f64, t2240: f64, t2241: f64, t605: f64, t645: f64, t86: f64, t112: f64, t111: f64, t649: f64, t671: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2289, t2291, t2296, t2298, t2304, t2307) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk441(t40, t632, t73, t52, t636, t76, t2244, t2250, t634, t638, t72, t2245, t2252, t2255, t2284, t609, t629, t642, t66, t80);
        let (t2311, t2312, t2314) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk442(t5, t2233, t2235, t2240, t2241, t2307, t605, t645, t86, t112, t111, t649);
        let t2319 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk443(t671);
    (t2289, t2291, t2296, t2298, t2304, t2307, t2311, t2312, t2314, t2319)
}
