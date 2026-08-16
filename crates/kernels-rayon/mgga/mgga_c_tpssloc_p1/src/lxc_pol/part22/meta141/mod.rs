//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta141 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk910;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk911;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk912;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk913;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk914;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta141(t1527: f64, t865: f64, t2718: f64, t2627: f64, t68: f64, t226: f64, t1509: f64, t252: f64, t4182: f64, t1510: f64, t2732: f64, t4234: f64, t860: f64, t814: f64, t829: f64, t1519: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t4273 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk910(t1527, t865, t2718);
        let (t4280, t4281) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk911(t2627, t68, t226);
        let t4282 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk912(t1509, t252);
        let (t4283, t4286, t4288, t4290, t4291) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk913(t4182, t4282, t1510, t2732, t4234, t860, t68, t814, t226);
        let (t4292, t4295) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk914(t4282, t829, t1519, t814);
    (t4273, t4280, t4281, t4282, t4283, t4286, t4288, t4290, t4291, t4292, t4295)
}
