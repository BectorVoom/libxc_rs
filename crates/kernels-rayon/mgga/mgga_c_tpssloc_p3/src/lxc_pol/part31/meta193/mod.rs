//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta193 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk863;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk864;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta193(t25: f64, t184: f64, t5151: f64, t17: f64, t1787: f64, t750: f64, t1408: f64, t3704: f64, t1298: f64, t2: f64, t584: f64, t606: f64, t1649: f64, t3711: f64, zeta_threshold: f64, t28: f64, t1302: f64, t1081: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t5166, t5167, t5168, t5169, t5170, t5177, t5178) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk863(t25, t184, t5151, t17, t1787, t750, t1408, t3704, t1298, t2, t584, t606, t1649, t3711, zeta_threshold);
        let t5187 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk864(t28, t1302, t2, t1081, t5178, t584, t5177, zeta_threshold);
    (t5166, t5167, t5168, t5169, t5170, t5178, t5187)
}
