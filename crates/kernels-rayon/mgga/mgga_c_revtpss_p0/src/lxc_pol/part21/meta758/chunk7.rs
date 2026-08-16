//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2673/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2673(t10069: f64, t14207: f64, t40921: f64, t5737: f64, t225: f64, t2453: f64, t136: f64, t137: f64, t1398: f64, t14140: f64, t2438: f64, t4003: f64) -> (f64, f64, f64, f64) {
    let t49176 = t10069 * t14207;
    let t49177 = 0.21951497276451705329e-1_f64 * t49176;
    let t49178 = t40921 * t5737;
    let t49180 = t2453 * t225;
    let t49186 = t49180 * t14140 * t4003 * t136 * t137 * t2438 * t1398;
    (t49177, t49178, t49180, t49186)
}
