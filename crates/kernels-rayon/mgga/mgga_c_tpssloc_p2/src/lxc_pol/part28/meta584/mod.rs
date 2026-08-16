//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta584 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1873;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1874;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta584(t13450: f64, t1888: f64, t6646: f64, t23110: f64, t23185: f64, t4292: f64, t25288: f64, t81591: f64, t234: f64, t4265: f64, t6552: f64, t6637: f64, t776: f64, t25237: f64, t23168: f64, t25307: f64, t10007: f64, t22986: f64, t4282: f64, t25287: f64, t81651: f64, t13401: f64, t22996: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87578, t87581, t87583, t87589) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1873(t13450, t1888, t6646, t23110, t23185, t4292, t25288, t81591, t234, t4265, t6552, t6637, t776);
        let (t87601, t87603, t87609, t87612, t87615) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1874(t23110, t23185, t25237, t23168, t25307, t10007, t22986, t4282, t6646, t25287, t81651, t13401, t1888, t22996);
    (t87578, t87581, t87583, t87589, t87601, t87603, t87609, t87612, t87615)
}
