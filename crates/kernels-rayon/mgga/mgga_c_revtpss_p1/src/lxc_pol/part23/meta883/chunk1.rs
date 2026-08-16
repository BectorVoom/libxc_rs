//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2795/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2795(t10139: f64, t136: f64, t2457: f64, t6874: f64, t6844: f64, t14145: f64, t14171: f64, t1882: f64, t2482: f64, t10069: f64, t22361: f64, t22365: f64) -> (f64, f64, f64, f64, f64) {
    let t75123 = t10139 * t6874 * t136 * t2457;
    let t75128 = t10139 * t6844 * t136 * t2457;
    let t75141 = t2482 * t14171 * t1882 * t14145;
    let t75145 = t10069 * t22361;
    let t75147 = t10069 * t22365;
    (t75123, t75128, t75141, t75145, t75147)
}
