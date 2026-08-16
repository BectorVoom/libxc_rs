//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1526/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1526(t11132: f64, t11337: f64, t2966: f64, t944: f64, t302: f64) -> (f64, f64, f64, f64) {
    let t11422 = 0.16068111111111111111e1_f64 * t11132;
    let t11423 = 0.46308888888888888888e0_f64 * t11337;
    let t11449 = 1.0_f64 / t2966 / t944;
    let t11450 = t302 * t11449;
    (t11422, t11423, t11449, t11450)
}
