//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta320 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1080;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta320(t1653: f64, t6219: f64, t3578: f64, t1735: f64, t5971: f64, t11668: f64, t5979: f64, t1730: f64, t6164: f64, t2130: f64, t47: f64, t479: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22153, t22154, t22157, t22158, t22161, t22162, t22169, t22173, t22174) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1080(t1653, t6219, t3578, t1735, t5971, t11668, t5979, t1730, t6164, t2130, t47, t479);
    (t22153, t22154, t22157, t22158, t22161, t22162, t22169, t22173, t22174)
}
