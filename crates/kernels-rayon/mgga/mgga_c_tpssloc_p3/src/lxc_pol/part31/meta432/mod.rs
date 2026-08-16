//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta432 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1564;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1565;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1566;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta432(t2627: f64, t6604: f64, t6579: f64, t6649: f64, t1879: f64, t22715: f64, t1906: f64, t6652: f64, t794: f64, t6562: f64, t6547: f64, t6653: f64, t22723: f64, t6561: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t22996 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1564(t2627, t6604);
        let (t23003, t23012) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1565(t6579, t6649, t1879, t22715);
        let (t23013, t23025, t23026, t23029, t23030) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1566(t1906, t23012, t6652, t794, t6562, t6547, t6653, t22723, t6561);
    (t22996, t23003, t23012, t23013, t23025, t23026, t23029, t23030)
}
