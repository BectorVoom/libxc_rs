//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta251 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1374;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1375;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1376;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1377;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1378;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta251(t2244: f64, t751: f64, t2658: f64, t9853: f64, t9859: f64, t9911: f64, t9914: f64, t9917: f64, t9921: f64, t9923: f64, t9925: f64, t9928: f64, t9931: f64, t225: f64, t9725: f64, t9877: f64, t9908: f64, t1891: f64, t68: f64, t9458: f64, t776: f64, t845: f64, t2553: f64, t824: f64, t9516: f64, t228: f64, t230: f64, t2667: f64, t2672: f64, t2675: f64, t4225: f64, t822: f64, t825: f64, t232: f64, t819: f64, t820: f64, t2571: f64, t2618: f64, t2643: f64, t2649: f64, t2686: f64, t817: f64, t9642: f64, t9649: f64, t9653: f64, t9657: f64, t9663: f64, t9668: f64, t9672: f64, t9675: f64, t9679: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9932, t9933, t9934, t9935) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1374(t2244, t751, t2658, t9853, t9859, t9911, t9914, t9917, t9921, t9923, t9925, t9928, t9931);
        let (t9938, t9947, t9951, t9954) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1375(t225, t9725, t9877, t9908, t9935, t1891, t68, t9458, t776, t845, t2553, t824, t9516);
        let t9957 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1376(t228, t230, t2667, t2672, t2675, t4225, t822, t825, t9938, t9947, t9951, t9954);
        let t9958 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1377(t232, t9957);
        let (t9960, t9963) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1378(t819, t820, t9958, t2571, t2618, t2643, t2649, t2686, t817, t9642, t9649, t9653, t9657, t9663, t9668, t9672, t9675, t9679);
    (t9932, t9933, t9934, t9938, t9947, t9951, t9954, t9957, t9958, t9960, t9963)
}
