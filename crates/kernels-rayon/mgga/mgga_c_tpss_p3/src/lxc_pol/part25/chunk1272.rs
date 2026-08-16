//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1272/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1272(t18495: f64, t5736: f64, t10179: f64, t1771: f64, t5570: f64, t10164: f64, t1765: f64, t18444: f64, t339: f64, t789: f64, t10085: f64, t64: f64) -> (f64, f64, f64, f64, f64) {
    let t60649 = t5736 * t18495;
    let t60653 = t1771 * t5570 * t10179;
    let t60684 = t1765 * t10164;
    let t60695 = t339 * t18444 * t789;
    let t60698 = t10085 * t64;
    (t60649, t60653, t60684, t60695, t60698)
}
