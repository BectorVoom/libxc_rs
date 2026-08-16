//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 222/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk222(t158: f64, t725: f64, t162: f64, t691: f64, t187: f64, t192: f64, t72: f64, t186: f64, t650: f64, t660: f64) -> (f64, f64, f64, f64, f64) {
    let t726 = t158 * t725;
    let t727 = t691 * t162;
    let t729 = 0.19751673498613801407e-1_f64 * t727 * t187;
    let t730 = t192 * t72;
    let t732 = t660 * t650 * t186;
    (t726, t727, t729, t730, t732)
}
