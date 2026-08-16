//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta476 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1689;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1690;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta476(t25373: f64, t25374: f64, t1530: f64, t606: f64, t25: f64, t4303: f64, t1408: f64, t776: f64, t868: f64, t28: f64, t870: f64, t4255: f64, t16596: f64, t23788: f64, t1081: f64, t1484: f64, t4119: f64, t25365: f64, t10143: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25375, t25377, t25381, t25385, t25392, t25891, t25892) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1689(t25373, t25374, t1530, t606, t25, t4303, t1408, t776, t868, t28, t870, t4255);
        let (t25898, t25901, t25905, t25921, t25927) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1690(t16596, t23788, t1081, t1484, t28, t4119, t25365, t10143);
    (t25375, t25377, t25381, t25385, t25392, t25891, t25892, t25898, t25901, t25905, t25921, t25927)
}
