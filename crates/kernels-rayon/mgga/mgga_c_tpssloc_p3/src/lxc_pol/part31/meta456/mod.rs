//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta456 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1607;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1608;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta456(t1512: f64, t23041: f64, t4166: f64, t6613: f64, t831: f64, t23053: f64, t4236: f64, t6614: f64, t1878: f64, t23033: f64, t221: f64, t4255: f64, t253: f64, t254: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25144, t25146, t25147, t25149, t25151, t25154, t25155) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1607(t1512, t23041, t4166, t6613, t831, t23053, t4236, t6614, t1878, t23033, t221, t4255);
        let (t25156, t25168) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1608(t25154, t25155, t253, t254);
    (t25144, t25146, t25147, t25149, t25151, t25154, t25155, t25156, t25168)
}
