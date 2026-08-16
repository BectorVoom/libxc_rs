//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta687 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2129;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta687(t19451: f64, t6534: f64, t1458: f64, t4025: f64, t1873: f64, t55943: f64, t19456: f64, t7467: f64, t26135: f64, t4028: f64, t5493: f64, t649: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t96681, t96683, t96685, t96704, t96706, t96708, t96709) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2129(t19451, t6534, t1458, t4025, t1873, t55943, t19456, t7467, t26135, t4028, t5493, t649);
    (t96681, t96683, t96685, t96704, t96706, t96708, t96709)
}
