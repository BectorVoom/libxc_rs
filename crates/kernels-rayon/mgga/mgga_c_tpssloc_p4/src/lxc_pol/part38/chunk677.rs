//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 677/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk677(t1118: f64, t3307: f64, t1099: f64, t1097: f64, t409: f64) -> (f64, f64, f64, f64, f64) {
    let t3308 = t3307 * t1118;
    let t3310 = 1.0_f64 * t1099 * t3308;
    let t3311 = t1097 * t1097;
    let t3312 = 1.0_f64 / t3311;
    let t3313 = t409 * t3312;
    (t3308, t3310, t3311, t3312, t3313)
}
