//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta459 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1342;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1343;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1344;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1345;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta459(t2775: f64, t75847: f64, t123: f64, t882: f64, t20217: f64, t4342: f64, t59688: f64, t59694: f64, t68444: f64, t68446: f64, t68448: f64, t68494: f64, t68498: f64, t76610: f64, t76614: f64, t76618: f64, t324: f64, t76602: f64, t300: f64, t1589: f64, t69012: f64, t5774: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t76620, t76622) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1342(t2775, t75847, t123, t882);
        let (t76624, t76626) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1343(t20217, t4342, t123, t882);
        let t76630 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1344(t59688, t59694, t68444, t68446, t68448, t68494, t68498, t76610, t76614, t76618, t76622, t76626);
        let (t76632, t76634, t76636, t76637) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1345(t324, t76602, t76630, t300, t1589, t69012, t5774);
    (t76620, t76622, t76624, t76626, t76632, t76634, t76636, t76637)
}
