//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta83 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk485;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk486;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk487;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk488;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk489;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk490;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta83(t815: f64, t835: f64, t812: f64, t242: f64, t67: f64, t845: f64, t246: f64, t152: f64, t32: f64, t181: f64, t204: f64, t686: f64, t756: f64, t68: f64, t20: f64, t61: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2638, t2639) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk485(t815, t835, t812);
        let (t2642, t2643) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk486(t242, t815, t812);
        let (t2644, t2645) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk487(t67, t845, t246);
        let t2658 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk488(t152, t32);
        let t2663 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk489(t181, t204, t686);
        let (t2665, t2671, t2690) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk490(t2663, t756, t68, t845, t20, t61);
    (t2638, t2639, t2642, t2643, t2644, t2645, t2658, t2663, t2665, t2671, t2690)
}
