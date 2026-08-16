//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta466 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1364;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1365;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1366;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1367;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta466(t42086: f64, t59688: f64, t59694: f64, t76574: f64, t76578: f64, t76583: f64, t76591: f64, t76599: f64, t76614: f64, t76622: f64, t76893: f64, t76896: f64, t76909: f64, t76915: f64, t77072: f64, t894: f64, t2798: f64, t77041: f64, t41942: f64, t77075: f64, t42087: f64, t47787: f64, t76587: f64, t76595: f64, t76610: f64, t76618: f64, t76626: f64, t76899: f64, t76903: f64, t76906: f64, t76912: f64, t77037: f64, t77082: f64, t893: f64, t913: f64, t5791: f64, t5811: f64, t959: f64, t13727: f64, t21315: f64, t2842: f64, t5695: f64, t5726: f64, t1557: f64, t21299: f64, t2792: f64, t10661: f64, t5730: f64, t13520: f64, t21318: f64, t1556: f64, t69347: f64, t5790: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t77097 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1364(t42086, t59688, t59694, t76574, t76578, t76583, t76591, t76599, t76614, t76622, t76893, t76896, t76909, t76915);
        let (t77102, t77105, t77107, t77114) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1365(t77072, t894, t2798, t77041, t41942, t77075, t42087, t47787, t76587, t76595, t76610, t76618, t76626, t76899, t76903, t76906, t76912);
        let (t77119, t77122, t77124, t77127) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1366(t77037, t77082, t77097, t77114, t893, t913, t5791, t5811, t959, t13727, t21315, t2842, t5695, t5726);
        let (t77130, t77133, t77135, t77138, t77139) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1367(t1557, t21299, t2792, t10661, t5726, t5730, t13520, t21318, t1556, t2842, t69347, t5790);
    (t77102, t77105, t77107, t77119, t77122, t77124, t77127, t77130, t77133, t77135, t77138, t77139)
}
