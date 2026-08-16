//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta335 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1104;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1105;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta335(t591: f64, t9688: f64, t2386: f64, t240: f64, t2385: f64, t2558: f64, t686: f64, t685: f64, t120: f64, t118: f64, t123: f64, t116: f64, t268: f64, t8705: f64, t9701: f64, t2397: f64, t693: f64, t119: f64, t133: f64, t39273: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39275, t39277, t39278, t39280, t39281, t39283, t39284, t39289) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1104(t591, t9688, t2386, t240, t2385, t2558, t686, t685, t120, t118, t123, t116, t268, t8705);
        let (t39291, t39293, t39295, t39298, t39300) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1105(t591, t9701, t2397, t39277, t39280, t693, t119, t133, t240, t39273, t39275, t39278, t39281, t39284, t39289);
    (t39275, t39278, t39281, t39283, t39284, t39289, t39291, t39293, t39295, t39298, t39300)
}
