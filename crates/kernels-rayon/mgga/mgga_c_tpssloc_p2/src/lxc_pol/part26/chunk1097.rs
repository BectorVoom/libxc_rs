//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1097/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1097(t16312: f64, t550: f64, t1339: f64, t22827: f64, t242: f64, t6943: f64, t1336: f64) -> (f64, f64, f64, f64, f64) {
    let t22828 = t16312 * t550;
    let t22829 = t1339 * t22828;
    let t22830 = t22827 * t22829;
    let t22832 = t6943 * t242;
    let t22833 = t1336 * t22832;
    (t22828, t22829, t22830, t22832, t22833)
}
