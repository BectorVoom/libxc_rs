//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 801/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk801(t1268: f64, t2314: f64, t2363: f64, t5113: f64, t671: f64, t9347: f64, t9348: f64, t9351: f64, t9416: f64, t195: f64, t40: f64, t2433: f64, t607: f64) -> (f64, f64, f64) {
    let t9419 = 2.0_f64 * t1268 * t9416 + 6.0_f64 * t2314 * t2363 + 6.0_f64 * t2363 * t5113 + 6.0_f64 * t671 * t9348 + t9347 + 6.0_f64 * t9351;
    let t9427 = 1.0_f64 / t195 / t40;
    let t9430 = t2433 * t607;
    (t9419, t9427, t9430)
}
