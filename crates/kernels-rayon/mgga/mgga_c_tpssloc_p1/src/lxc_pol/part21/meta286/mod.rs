//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta286 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1580;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1581;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1582;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1583;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1584;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1585;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1586;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta286(t10335: f64, t221: f64, t339: f64, t2955: f64, t995: f64, t3069: f64, t3180: f64, t3036: f64, t67: f64, t3067: f64, t3186: f64, t3062: f64, t820: f64, t3200: f64, t3051: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10383, t10385, t10388, t10390) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1580(t10335, t221, t339, t2955, t995, t3069, t3180);
        let t10401 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1581(t3036, t67);
        let t10402 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1582(t10401, t3067);
        let t10403 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1583(t10402, t3186);
        let t10408 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1584(t3062, t820);
        let t10413 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1585(t10402, t3200);
        let t10422 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1586(t3051, t820);
    (t10383, t10385, t10388, t10390, t10401, t10402, t10403, t10408, t10413, t10422)
}
