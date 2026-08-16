//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1733/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1733(t16710: f64, t16712: f64, t1737: f64, t3451: f64, t1160: f64, t5117: f64, t3476: f64) -> (f64, f64, f64, f64, f64) {
    let t17010 = 0.2283111111111111111e-1_f64 * t16710;
    let t17011 = 0.11415555555555555555e-1_f64 * t16712;
    let t17023 = t1737 * t3451;
    let t17026 = t5117 * t1160;
    let t17032 = t1737 * t3476;
    (t17010, t17011, t17023, t17026, t17032)
}
