//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta180 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk828;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk829;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk830;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta180(t9798: f64, t9860: f64, t157: f64, t153: f64, t2371: f64, t2531: f64, t2528: f64, t2517: f64, t607: f64, t707: f64, t2652: f64, t2663: f64, t181: f64, t686: f64, t781: f64, t756: f64, t9727: f64, t9780: f64, t9789: f64, t9793: f64, t9797: f64, t118: f64, t753: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9861, t9862, t9863, t9865, t9867, t9868, t9870, t9871) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk828(t9798, t9860, t157, t153, t2371, t2531, t2528, t2517, t607, t707, t2652, t2663);
        let (t9872, t9874, t9876, t9877) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk829(t9871, t181, t686, t781, t756, t9727, t9780, t9789, t9793, t9797, t9863, t9865, t9867, t9870);
        let t9879 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk830(t118, t753);
    (t9861, t9862, t9863, t9865, t9867, t9868, t9870, t9872, t9874, t9876, t9877, t9879)
}
