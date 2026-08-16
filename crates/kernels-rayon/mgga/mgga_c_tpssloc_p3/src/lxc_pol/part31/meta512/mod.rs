//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta512 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1708;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta512(t28383: f64, t6591: f64, t23056: f64, t5568: f64, t1894: f64, t236: f64, t5527: f64, t23078: f64, t1484: f64, t1509: f64, t232: f64, t815: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t28384, t28386, t28389, t28390, t28395, t28396) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1708(t28383, t6591, t23056, t5568, t1894, t236, t5527, t23078, t1484, t1509, t232, t815);
    (t28384, t28386, t28389, t28390, t28395, t28396)
}
