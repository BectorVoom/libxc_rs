//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta558 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2001;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta558(t111: f64, t12723: f64, t1406: f64, t9238: f64, t12566: f64, t604: f64, t2239: f64, t3951: f64, t13034: f64, t225: f64, t10109: f64, t1527: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t45632, t45844, t46099, t46104, t46452, t46488) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2001(t111, t12723, t1406, t9238, t12566, t604, t2239, t3951, t13034, t225, t10109, t1527);
    (t45632, t45844, t46099, t46104, t46452, t46488)
}
