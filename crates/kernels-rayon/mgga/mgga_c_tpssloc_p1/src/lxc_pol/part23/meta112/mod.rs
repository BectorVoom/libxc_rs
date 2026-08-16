//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta112 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk589;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk590;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk591;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta112(t226: f64, t4280: f64, t1509: f64, t252: f64, t68: f64, t814: f64, t1519: f64, t1530: f64, t870: f64, t193: f64, t200: f64, t1484: f64, t262: f64, t1540: f64, t690: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4281, t4282, t4290, t4291, t4295, t4310) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk589(t226, t4280, t1509, t252, t68, t814, t1519, t1530, t870);
        let t4314 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk590(t193, t200);
        let (t4315, t4335) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk591(t1484, t262, t1540, t690);
    (t4281, t4282, t4290, t4291, t4295, t4310, t4314, t4315, t4335)
}
