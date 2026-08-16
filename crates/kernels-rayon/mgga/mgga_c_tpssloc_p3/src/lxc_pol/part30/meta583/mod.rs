//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1962;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta583(t2239: f64, t5385: f64, t111: f64, t19449: f64, t19644: f64, t225: f64, t20038: f64, t20032: f64, t20040: f64, t19635: f64, t20048: f64, t1351: f64, t6414: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t55921, t55943, t56422, t56434, t56580, t56596, t56607, t56640, t56812) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1962(t2239, t5385, t111, t19449, t19644, t225, t20038, t20032, t20040, t19635, t20048, t1351, t6414);
    (t55921, t55943, t56422, t56434, t56580, t56596, t56607, t56640, t56812)
}
