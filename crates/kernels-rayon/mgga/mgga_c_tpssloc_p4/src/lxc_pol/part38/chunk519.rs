//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 519/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk519(t2186: f64, t3: f64, t1401: f64, t2180: f64, t577: f64, t11: f64, t2: f64, t584: f64) -> (f64, f64, f64, f64) {
    let t2187 = t3 * t2186;
    let t2193 = 0.45e1_f64 * t2186 * t577 + 0.135e2_f64 * t1401 * t2180;
    let t2218 = 0.174e1_f64 * t11;
    let t2219 = t2 * t584;
    (t2187, t2193, t2218, t2219)
}
