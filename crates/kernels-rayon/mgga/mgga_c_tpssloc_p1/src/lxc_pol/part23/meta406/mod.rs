//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta406 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1219;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta406(t3331: f64, t6031: f64, t11282: f64, t6084: f64, t11292: f64, t4899: f64, t6138: f64, t6144: f64, t11588: f64, t1887: f64, t337: f64, t5416: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t64292, t64451, t64537, t64644, t64648, t64763, t64779, t64811) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1219(t3331, t6031, t11282, t6084, t11292, t4899, t6138, t6144, t11588, t1887, t337, t5416);
    (t64292, t64451, t64537, t64644, t64648, t64763, t64779, t64811)
}
