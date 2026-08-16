//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1100/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1100(t1307: f64, t1339: f64, t22827: f64, t3856: f64, t12251: f64, t12289: f64, t6936: f64, t22811: f64, t61: f64, t133: f64, t1995: f64, t6933: f64) -> (f64, f64, f64, f64) {
    let t80947 = t22827 * t1339 * t3856 * t1307;
    let t80950 = t6936 * t12289 * t12251;
    let t80953 = 1.0_f64 / t61 / t22811;
    let t80956 = t80953 * t1995 * t133 * t6933;
    (t80947, t80950, t80953, t80956)
}
