//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 601/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk601(t3358: f64, t3236: f64, t1143: f64, t1147: f64, t1146: f64, t445: f64) -> (f64, f64, f64, f64) {
    let t3359 = 1.0_f64 / t3358;
    let t3363 = 0.12361111111111111111e-1_f64 * t3236;
    let t3371 = t1143 * t1147;
    let t3374 = t1146 * t445;
    let t3375 = 1.0_f64 / t3374;
    (t3359, t3363, t3371, t3375)
}
