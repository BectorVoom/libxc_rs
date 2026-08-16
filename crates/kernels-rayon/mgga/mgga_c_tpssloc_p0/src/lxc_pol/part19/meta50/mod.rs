//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta50 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk328;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk329;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk330;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk331;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk332;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk333;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta50(t1004: f64, t369: f64, t191: f64, t349: f64, t68: f64, t361: f64, t363: f64, t336: f64, t371: f64, t368: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1005, t1008, t1009) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk328(t1004, t369, t191);
        let (t1010, t1011) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk329(t1009, t349, t68);
        let (t1012, t1013, t1014) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk330(t1010, t1011, t361);
        let t1015 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk331(t1014, t363);
        let t1017 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk332(t336, t371);
        let (t1019, t1020) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk333(t1017, t368, t1015, t1012);
    (t1005, t1008, t1009, t1010, t1011, t1012, t1013, t1014, t1015, t1017, t1019, t1020)
}
