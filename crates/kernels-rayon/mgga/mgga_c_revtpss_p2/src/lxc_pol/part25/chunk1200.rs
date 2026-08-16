//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1200/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1200(t1949: f64, t2718: f64, t198: f64, t1993: f64, t11064: f64, t30: f64, t3046: f64, t7143: f64, t25515: f64, t4890: f64, t3299: f64, t3317: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27357 = t2718 * t1949;
    let t27382 = t198 * t1993;
    let t27383 = t11064 * t30;
    let t27415 = t3046 * t7143;
    let t27492 = t25515 * t4890;
    let t27493 = t3299 * t27492;
    let t27498 = t3317 * t27492;
    (t27357, t27382, t27383, t27415, t27492, t27493, t27498)
}
