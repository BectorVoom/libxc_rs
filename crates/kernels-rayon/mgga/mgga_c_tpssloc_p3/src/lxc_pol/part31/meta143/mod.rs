//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta143 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk732;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk733;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk734;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk735;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta143(t2218: f64, t2221: f64, t2225: f64, t2232: f64, t1406: f64, t604: f64, t1437: f64, t645: f64, t1409: f64, t607: f64, t25: f64, t28: f64, t65: f64, t2219: f64, zeta_threshold: f64, t31: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t3951, t3953) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk732(t2218, t2221, t2225, t2232, t1406, t604);
        let (t3958, t3961) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk733(t1437, t645, t1409, t607);
        let (t3962, t3966) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk734(t25, t28, t3961, t65, t2219, zeta_threshold);
        let t3967 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk735(t31, t3966);
    (t3951, t3953, t3958, t3961, t3962, t3966, t3967)
}
