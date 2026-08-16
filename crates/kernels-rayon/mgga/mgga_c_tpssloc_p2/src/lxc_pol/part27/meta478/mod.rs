//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta478 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1850;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta478(t1014: f64, t23602: f64, t1011: f64, t360: f64, t3187: f64, t3192: f64, t6800: f64, t6799: f64, t225: f64, t6733: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t23603, t23604, t23605, t23606, t23609, t23610, t23613) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1850(t1014, t23602, t1011, t360, t3187, t3192, t6800, t6799, t225, t6733);
    (t23603, t23604, t23605, t23606, t23609, t23610, t23613)
}
