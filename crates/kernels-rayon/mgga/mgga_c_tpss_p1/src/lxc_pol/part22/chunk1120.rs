//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1120/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1120(t1125: f64, t12367: f64, t3062: f64, t4258: f64, t1114: f64, t581: f64, t4051: f64, t3068: f64, t1113: f64, t1561: f64, t1014: f64, t450: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12368 = t1125 * t12367;
    let t12371 = t4258 * t3062 / 432.0_f64;
    let t12372 = t1114 * t581;
    let t12373 = t4051 * t12372;
    let t12374 = t3068 * t12373;
    let t12377 = t1561 * t1113;
    let t12378 = t450 * t1014;
    (t12368, t12371, t12372, t12374, t12377, t12378)
}
