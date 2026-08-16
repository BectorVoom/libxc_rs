//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta487 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1794;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1795;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta487(t25224: f64, t6555: f64, t6552: f64, t1911: f64, t4300: f64, t2718: f64, t1519: f64, t828: f64, t232: f64, t6646: f64, t1888: f64, t13384: f64, t23110: f64, t7524: f64, t23185: f64, t234: f64, t6604: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25229, t25230, t25233, t25237, t25238, t25239, t25241) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1794(t25224, t6555, t6552, t1911, t4300, t2718, t1519, t828, t232, t6646, t1888, t13384);
        let (t25242, t25243, t25245, t25246, t25248) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1795(t25241, t6646, t1888, t23110, t7524, t23185, t234, t6604);
    (t25229, t25230, t25233, t25237, t25238, t25239, t25241, t25242, t25243, t25245, t25246, t25248)
}
