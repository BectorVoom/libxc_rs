//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1182/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1182(t1761: f64, t30644: f64, t5807: f64, t7822: f64, t6153: f64, t6157: f64, t7647: f64, t1713: f64, t31491: f64, t7381: f64, t1165: f64, t34248: f64, t5532: f64, t7564: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40467 = t30644 * t1761;
    let t40469 = t7822 * t5807;
    let t40472 = t7822 * t6153;
    let t40474 = t7647 * t6157;
    let t40477 = t31491 * t7381 * t1713;
    let t40481 = t7564 * t1165 * t34248 * t5532;
    (t40467, t40469, t40472, t40474, t40477, t40481)
}
