//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1084/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1084(t25412: f64, t26481: f64, t25411: f64, t2466: f64, t25387: f64, t2061: f64, t2771: f64, t25317: f64, t7398: f64, t886: f64, t7071: f64, t2062: f64, t867: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26482 = t26481 * t25412;
    let t26483 = t25411 * t26482;
    let t26485 = t26481 * t2466;
    let t26486 = t25387 * t26485;
    let t26488 = t2061 * t2771;
    let t26489 = t25317 * t26488;
    let t26492 = t7398 * t886;
    let t26493 = t7071 * t26492;
    let t26496 = t2062 * t867;
    (t26482, t26483, t26485, t26486, t26488, t26489, t26492, t26493, t26496)
}
