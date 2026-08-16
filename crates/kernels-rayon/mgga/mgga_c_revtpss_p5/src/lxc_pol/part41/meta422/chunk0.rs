//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1479/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1479(t1518: f64, t2178: f64, t2681: f64, t64: f64, t10207: f64, t111: f64, t116: f64, t21813: f64, t5876: f64, t670: f64, t5891: f64, t665: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35739 = t1518 * t2178;
    let t46089 = t64 * t2681;
    let t46157 = 1.0_f64 / t10207 / t111;
    let t75439 = t21813 * t116;
    let t85360 = t5876 * t670;
    let t105872 = t5891 * t665;
    (t35739, t46089, t46157, t75439, t85360, t105872)
}
