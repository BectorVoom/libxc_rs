//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta622 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2064;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta622(t11801: f64, t7345: f64, t11708: f64, t24728: f64, t11713: f64, t11715: f64, t11717: f64, t2131: f64, t82985: f64, t24727: f64, t24732: f64, t7337: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t86136, t86140, t86146, t86154, t86164, t86167, t86171) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2064(t11801, t7345, t11708, t24728, t11713, t11715, t11717, t2131, t82985, t24727, t24732, t7337, sigma2);
    (t86136, t86140, t86146, t86154, t86164, t86167, t86171)
}
