//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta88 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk505;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk506;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk507;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk508;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk509;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk510;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta88(t815: f64, t835: f64, t812: f64, t831: f64, t242: f64, t67: f64, t845: f64, t246: f64, t120: f64, t828: f64, t232: f64, t776: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2638, t2639) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk505(t815, t835, t812);
        let (t2640, t2642, t2643) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk506(t2639, t831, t242, t815, t812);
        let (t2644, t2645) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk507(t67, t845, t246);
        let t2646 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk508(t120, t828);
        let t2647 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk509(t232, t776);
        let t2649 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk510(t2645, t2646, t2647);
    (t2638, t2639, t2640, t2642, t2643, t2644, t2645, t2646, t2647, t2649)
}
