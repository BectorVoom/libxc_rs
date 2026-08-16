//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta663 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2094;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta663(t27381: f64, t7294: f64, t24574: f64, t27383: f64, t7288: f64, t94490: f64, t27438: f64, t85639: f64, t225: f64, t27419: f64, t27427: f64, t5052: f64, t7284: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t94584, t94628, t94631, t94648, t94656, t94676, t94680) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2094(t27381, t7294, t24574, t27383, t7288, t94490, t27438, t85639, t225, t27419, t27427, t5052, t7284);
    (t94584, t94628, t94631, t94648, t94656, t94676, t94680)
}
