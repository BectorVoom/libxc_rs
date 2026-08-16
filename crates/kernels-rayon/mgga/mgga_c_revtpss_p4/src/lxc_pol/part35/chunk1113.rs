//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1113/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1113(t233: f64, t41077: f64, t7056: f64, t9646: f64, t1954: f64, t39643: f64, t2453: f64, t251: f64, t25410: f64, t25304: f64, t25374: f64, t68: f64, t785: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t93118 = t41077 * t233;
    let t93134 = t9646 * t7056;
    let t93139 = t1954 * t39643;
    let t93140 = t93139 * t7056;
    let t93169 = t2453 * t251;
    let t93170 = t93169 * t25410;
    let t93189 = t25304 * t251;
    let t93190 = t93189 * t25374;
    let t93238 = t68 * t785;
    (t93118, t93134, t93139, t93140, t93169, t93170, t93189, t93190, t93238)
}
