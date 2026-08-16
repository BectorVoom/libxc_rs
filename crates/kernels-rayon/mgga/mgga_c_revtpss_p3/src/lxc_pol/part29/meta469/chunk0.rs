//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1730/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1730(t25387: f64, t26485: f64, t2061: f64, t2771: f64, t25317: f64, t7398: f64, t886: f64, t7071: f64, t2062: f64, t867: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26486 = t25387 * t26485;
    let t26488 = t2061 * t2771;
    let t26489 = t25317 * t26488;
    let t26492 = t7398 * t886;
    let t26493 = t7071 * t26492;
    let t26496 = t2062 * t867;
    let t26497 = t786 * t26496;
    (t26486, t26488, t26489, t26492, t26493, t26496, t26497)
}
