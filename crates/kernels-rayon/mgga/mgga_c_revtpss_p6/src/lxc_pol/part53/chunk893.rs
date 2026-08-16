//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 893/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk893(t1580: f64, t7014: f64, t689: f64, t27279: f64, t7058: f64, t72: f64, t7769: f64, t686: f64, t25375: f64, t25387: f64, t1559: f64, t886: f64) -> (f64, f64, f64, f64, f64) {
    let t27334 = t7014 * t1580;
    let t27335 = t689 * t27334;
    let t27338 = t7058 * t27279;
    let t27340 = t7769 * t72;
    let t27341 = t27340 * t686;
    let t27342 = t25375 * t27341;
    let t27344 = t25387 * t27341;
    let t27349 = t1559 * t886;
    (t27335, t27338, t27342, t27344, t27349)
}
