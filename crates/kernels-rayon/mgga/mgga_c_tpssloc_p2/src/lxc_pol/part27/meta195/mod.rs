//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta195 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1003;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1004;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta195(t4343: f64, t882: f64, t123: f64, t3966: f64, t883: f64, t2765: f64, t2766: f64, t4335: f64, t4340: f64, t291: f64, t1543: f64, t892: f64, t914: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4344, t4345, t4347) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1003(t4343, t882, t123, t3966, t883);
        let (t4348, t4349, t4351, t4353, t4354, t4356) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1004(t4347, t882, t123, t2765, t2766, t4335, t4340, t4345, t291, t1543, t892, t914);
    (t4344, t4345, t4347, t4348, t4349, t4351, t4353, t4354, t4356)
}
