//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta557 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1920;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1921;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta557(t28130: f64, t6976: f64, t22633: f64, t19743: f64, t3792: f64, t22897: f64, t1992: f64, t6347: f64, t6968: f64, t6637: f64, t6888: f64, t6330: f64, t22685: f64, t1799: f64, t26395: f64, t6415: f64, t6987: f64, t1336: f64, t1814: f64, t2013: f64, t22693: f64, t26381: f64, t26427: f64, t27082: f64, t27088: f64, t6378: f64, t7747: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28131, t28132, t28134, t28135, t28136, t28138, t28139, t28140, t28142) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1920(t28130, t6976, t22633, t19743, t3792, t22897, t1992, t6347, t6968, t6637, t6888, t6330);
        let (t28143, t28148, t28149, t28152, t28155) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1921(t28142, t6637, t22685, t1799, t26395, t6888, t6415, t6987, t1336, t1814, t2013, t22693, t26381, t26427, t27082, t27088, t28132, t28136, t28140, t6378, t7747);
    (t28131, t28134, t28135, t28138, t28139, t28142, t28143, t28148, t28149, t28152, t28155)
}
