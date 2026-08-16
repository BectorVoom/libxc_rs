//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta844 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3049;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3050;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3051;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3052;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3053;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3054;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3055;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta844(t12606: f64, t4723: f64, t123: f64, t3240: f64, t18226: f64, t690: f64, t18222: f64, t2250: f64, t5971: f64, t1088: f64, t18210: f64, t2244: f64, t4728: f64, t11147: f64, t5398: f64, t11145: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t63394, t63396) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3049(t12606, t4723, t123, t3240);
        let t63398 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3050(t18226, t690);
        let t63400 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3051(t18222, t690);
        let (t63402, t63404) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3052(t2250, t5971, t1088, t123);
        let (t63406, t63408) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3053(t18210, t2244, t1088, t123);
        let (t63410, t63412) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3054(t12606, t4728, t1088, t123);
        let (t63415, t63417) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3055(t11147, t2244, t5398, t11145, t123);
    (t63394, t63396, t63398, t63400, t63402, t63404, t63406, t63408, t63410, t63412, t63415, t63417)
}
