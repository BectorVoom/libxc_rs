//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta842 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3035;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3036;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3037;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3038;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3039;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3040;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3041;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta842(t18242: f64, t690: f64, t43748: f64, t50903: f64, t50905: f64, t50907: f64, t50919: f64, t50921: f64, t50948: f64, t50950: f64, t63327: f64, t63330: f64, t63332: f64, t63334: f64, t1089: f64, t55677: f64, t1088: f64, t123: f64, t3242: f64, t55723: f64, t3240: f64, t2394: f64, t5976: f64, t3247: f64, t11153: f64, t2244: f64, t5398: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t63336 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3035(t18242, t690);
        let t63346 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3036(t43748, t50903, t50905, t50907, t50919, t50921, t50948, t50950, t63327, t63330, t63332, t63334, t63336);
        let (t63353, t63355) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3037(t1089, t55677, t1088, t123);
        let (t63357, t63359) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3038(t3242, t55723, t123, t3240);
        let t63361 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3039(t2394, t5976);
        let (t63363, t63365) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3040(t3247, t55723, t1088, t123);
        let (t63368, t63370) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3041(t11153, t2244, t5398, t123, t3240);
    (t63336, t63346, t63353, t63355, t63357, t63359, t63361, t63363, t63365, t63368, t63370)
}
