//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta152 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk709;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk710;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta152(t300: f64, t6091: f64, t6064: f64, t1703: f64, t4869: f64, t1156: f64, t3375: f64, t6068: f64, t1164: f64, t1147: f64, t6084: f64, t3400: f64, t3403: f64, t338: f64, t5416: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6092, t6094, t6096, t6098, t6100, t6102, t6104, t6105) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk709(t300, t6091, t6064, t1703, t4869, t1156, t3375, t6068, t1164, t1147, t6084, t3400);
        let (t6106, t6108, t6109) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk710(t3403, t6105, t1164, t338, t5416);
    (t6092, t6094, t6096, t6098, t6100, t6102, t6104, t6105, t6106, t6108, t6109)
}
