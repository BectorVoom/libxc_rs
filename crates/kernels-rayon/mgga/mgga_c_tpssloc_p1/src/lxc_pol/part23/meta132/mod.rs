//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta132 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk641;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk642;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk643;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta132(t210: f64, t214: f64, t5527: f64, t5544: f64, t2562: f64, t2569: f64, t2571: f64, t2590: f64, t4124: f64, t4135: f64, t787: f64, t252: f64, t1492: f64, t1519: f64, t119: f64, t225: f64, t237: f64, t1509: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5550, t5555, t5558, t5559) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk641(t210, t214, t5527, t5544, t2562, t2569, t2571, t2590, t4124, t4135, t787, t252);
        let (t5561, t5567, t5568, t5572, t5575) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk642(t1492, t1519, t119, t5527, t210, t5544, t225, t5558);
        let (t5576, t5584) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk643(t237, t5575, t1509);
    (t5550, t5555, t5558, t5559, t5561, t5567, t5568, t5572, t5575, t5576, t5584)
}
