//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1403/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1403(t1770: f64, t5477: f64, t1248: f64, t17847: f64, t20956: f64, t17854: f64, t1280: f64, t20721: f64, t5284: f64, t5464: f64, t5332: f64, t1287: f64, t20856: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21579 = t1770 * t5477;
    let t21582 = t17847 * t1248;
    let t21583 = t20956 * t21582;
    let t21586 = t17854 * t1248;
    let t21587 = t20956 * t21586;
    let t21592 = t1280 * t20721;
    let t21595 = t5464 * t5284;
    let t21596 = t5332 * t21595;
    let t21599 = t20856 * t1287;
    (t21579, t21583, t21587, t21592, t21596, t21599)
}
