//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 456/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk456(t225: f64, t2633: f64, t73: f64, t853: f64, t2394: f64, t2430: f64, t832: f64, t227: f64, t229: f64, t830: f64, t833: f64) -> (f64, f64, f64, f64) {
    let t2634 = t2633 * t225;
    let t2638 = t73 * t853;
    let t2639 = t2638 * t2394;
    let t2642 = t832 * t2430;
    let t2645 = -12.0_f64 * t227 * t2639 + 3.0_f64 * t227 * t2642 - t229 * t2634 + 6.0_f64 * t830 * t833;
    (t2634, t2639, t2642, t2645)
}
