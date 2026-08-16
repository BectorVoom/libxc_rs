//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1148/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1148(t786: f64, t7998: f64, t867: f64, t26506: f64, t27213: f64, t103000: f64, t93371: f64, t25410: f64, t8011: f64, t93240: f64, t1580: f64, t2439: f64, t26434: f64) -> (f64, f64, f64, f64, f64) {
    let t103067 = t786 * t7998 * t867;
    let t103114 = t27213 * t26506;
    let t103122 = t93371 * t103000;
    let t103130 = t93240 * t25410 * t8011;
    let t103158 = t2439 * t26434 * t1580;
    (t103067, t103114, t103122, t103130, t103158)
}
