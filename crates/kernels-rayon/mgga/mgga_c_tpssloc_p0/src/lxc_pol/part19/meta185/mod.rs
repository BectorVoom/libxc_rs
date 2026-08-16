//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta185 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk837;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk838;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk839;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta185(t225: f64, t9725: f64, t9877: f64, t9908: f64, t9935: f64, t1891: f64, t68: f64, t9458: f64, t776: f64, t845: f64, t2553: f64, t824: f64, t9516: f64, t228: f64, t230: f64, t2667: f64, t2672: f64, t2675: f64, t4225: f64, t822: f64, t825: f64, t232: f64, t819: f64, t820: f64, t2571: f64, t2618: f64, t2643: f64, t2649: f64, t2686: f64, t817: f64, t9642: f64, t9649: f64, t9653: f64, t9657: f64, t9663: f64, t9668: f64, t9672: f64, t9675: f64, t9679: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9938, t9947, t9950, t9951, t9954) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk837(t225, t9725, t9877, t9908, t9935, t1891, t68, t9458, t776, t845, t2553, t824, t9516);
        let t9957 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk838(t228, t230, t2667, t2672, t2675, t4225, t822, t825, t9938, t9947, t9951, t9954);
        let (t9958, t9960, t9963) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk839(t232, t9957, t819, t820, t2571, t2618, t2643, t2649, t2686, t817, t9642, t9649, t9653, t9657, t9663, t9668, t9672, t9675, t9679);
    (t9938, t9947, t9950, t9951, t9954, t9957, t9958, t9960, t9963)
}
