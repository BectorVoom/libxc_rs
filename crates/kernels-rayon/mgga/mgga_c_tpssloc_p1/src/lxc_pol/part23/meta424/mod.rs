//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta424 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1252;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1253;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta424(t10480: f64, t21391: f64, t248: f64, t3101: f64, t1041: f64, t10457: f64, t21118: f64, t1020: f64, t21595: f64, t14511: f64, t17655: f64, t10883: f64, t21403: f64, t21130: f64, t42592: f64, t21594: f64, t376: f64, t10422: f64, t21519: f64, t3070: f64, t135: f64, t21561: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t70227, t70239, t70346, t70351, t70363) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1252(t10480, t21391, t248, t3101, t1041, t10457, t21118, t1020, t21595, t14511, t17655, t10883, t21403);
        let (t70389, t70391, t70404, t70497) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1253(t1041, t21130, t248, t42592, t21594, t376, t10422, t21519, t3070, t135, t21561, t973);
    (t70227, t70239, t70346, t70351, t70363, t70389, t70391, t70404, t70497)
}
