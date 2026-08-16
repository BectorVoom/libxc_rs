//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta113 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk592;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk593;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta113(t1409: f64, t2770: f64, t2775: f64, t1543: f64, t892: f64, t1547: f64, t2798: f64, t2815: f64, t1553: f64, t699: f64, t1561: f64, t923: f64, t1573: f64, t942: f64, t1580: f64, t2932: f64, t300: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4337, t4342, t4354, t4362, t4378, t4384, t4411) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk592(t1409, t2770, t2775, t1543, t892, t1547, t2798, t2815, t1553, t699, t1561, t923);
        let (t4449, t4475, t4483) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk593(t1573, t942, t1580, t2932, t300);
    (t4337, t4342, t4354, t4362, t4378, t4384, t4411, t4449, t4475, t4483)
}
