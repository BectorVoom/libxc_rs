//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1988/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1988(t233: f64, t41077: f64, t7056: f64, t9646: f64, t1949: f64, t22: f64, t25402: f64, t1954: f64, t39643: f64, t2470: f64, t25295: f64, t7058: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93118 = t41077 * t233;
    let t93134 = t9646 * t7056;
    let t93136 = t25402 * t1949 * t22;
    let t93138 = 0.43639970290213137151e-3_f64 * t93134 * t93136;
    let t93139 = t1954 * t39643;
    let t93140 = t93139 * t7056;
    let t93142 = 0.51727911450665971904e-3_f64 * t93140 * t93136;
    let t93150 = t25295 * t2470;
    let t93151 = t7058 * t93150;
    (t93118, t93138, t93139, t93142, t93150, t93151)
}
