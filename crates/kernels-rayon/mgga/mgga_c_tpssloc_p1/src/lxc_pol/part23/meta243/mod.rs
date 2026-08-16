//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta243 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk899;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk900;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk901;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk902;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta243(t1036: f64, t5905: f64, t4571: f64, t4644: f64, t1009: f64, t5848: f64, t1011: f64, t1019: f64, t10422: f64, t5908: f64, t3070: f64, t225: f64, t5915: f64, t1057: f64, t5972: f64, t690: f64, t11147: f64, t5392: f64, t11153: f64, t5976: f64, t5980: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18005, t18008, t18028, t18030, t18041, t18042, t18074) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk899(t1036, t5905, t4571, t4644, t1009, t5848, t1011, t1019, t10422, t5908, t3070, t225, t5915);
        let (t18086, t18203) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk900(t1057, t18028, t5972, t690);
        let (t18205, t18210, t18219) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk901(t11147, t5392, t11153, t5976, t690);
        let t18229 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk902(t5980, t690);
    (t18005, t18008, t18030, t18041, t18042, t18074, t18086, t18203, t18205, t18210, t18219, t18229)
}
