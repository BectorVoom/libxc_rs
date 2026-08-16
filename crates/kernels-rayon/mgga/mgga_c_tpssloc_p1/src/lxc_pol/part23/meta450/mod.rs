//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta450 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1297;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1298;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta450(t20825: f64, t46387: f64, t67099: f64, t46196: f64, t5660: f64, t193: f64, t202: f64, t2752: f64, t39316: f64, t39320: f64, t39373: f64, t39397: f64, t39400: f64, t39408: f64, t39411: f64, t40679: f64, t40685: f64, t40708: f64, t57960: f64, t46208: f64, t57992: f64, t1462: f64, t67181: f64, t16625: f64, t20947: f64, t2522: f64, t39463: f64, t39468: f64, t39472: f64, t39476: f64, t40714: f64, t40716: f64, t40721: f64, t40732: f64, t4310: f64, t4314: f64, t5544: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t75854, t75855, t75856, t75862) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1297(t20825, t46387, t67099, t46196, t5660, t193, t202, t2752, t39316, t39320, t39373, t39397, t39400, t39408, t39411, t40679, t40685, t40708);
        let (t75864, t75865, t75872, t75874, t75875) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1298(t57960, t46208, t57992, t1462, t67181, t16625, t20947, t2522, t39463, t39468, t39472, t39476, t40714, t40716, t40721, t40732, t4310, t4314, t5544);
    (t75854, t75855, t75856, t75862, t75864, t75865, t75872, t75874, t75875)
}
