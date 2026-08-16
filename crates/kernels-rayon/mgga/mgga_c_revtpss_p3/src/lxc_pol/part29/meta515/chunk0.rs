//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1837/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1837(t233: f64, t41077: f64, t1955: f64, t92888: f64, t7056: f64, t9646: f64, t1954: f64, t39643: f64, t2453: f64, t25309: f64, t25304: f64, t251: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t93118 = t41077 * t233;
    let t93126 = t1955 * t92888;
    let t93134 = t9646 * t7056;
    let t93139 = t1954 * t39643;
    let t93140 = t93139 * t7056;
    let t93157 = t2453 * t25309;
    let t93160 = t25304 * t25309;
    let t93169 = t2453 * t251;
    (t93118, t93126, t93134, t93139, t93140, t93157, t93160, t93169)
}
