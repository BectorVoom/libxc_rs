//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2804/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2804(t114: f64, t46143: f64, t46144: f64, t49698: f64, t49701: f64, t49818: f64, t75526: f64, t75540: f64, t75639: f64, t75641: f64, t75643: f64, t75929: f64, t116: f64, t22746: f64) -> (f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t75931 = piecewise3(t115, 0.0_f64, t46143 + 154.0_f64 / 27.0_f64 * t46144 + 154.0_f64 / 9.0_f64 * t49698 + t49701 - t49818 + 22.0_f64 / 3.0_f64 * t75639 + 6.0_f64 * t75641 - 4.0_f64 * t75643 - 11.0_f64 / 3.0_f64 * t75540 - 2.0_f64 * t75526 + t75929);
    let t75941 = t22746 * t116;
    (t75931, t75941)
}
