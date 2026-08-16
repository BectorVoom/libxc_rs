//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1151/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1151(t2247: f64, t26754: f64, t2282: f64, t55: f64, t2251: f64, t2258: f64, t25137: f64, t7571: f64, t72: f64, t1927: f64, t6977: f64, t7575: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26755 = t2247 * t26754;
    let t26776 = t55 * t2282;
    let t26781 = 5.0_f64 / 18.0_f64 * t26776 * t2251 - 5.0_f64 / 6.0_f64 * t7571 * t2258 - t25137;
    let t26782 = t26781 * t72;
    let t26783 = t26782 * t1927;
    let t26786 = t7575 * t6977;
    (t26755, t26776, t26781, t26782, t26783, t26786)
}
