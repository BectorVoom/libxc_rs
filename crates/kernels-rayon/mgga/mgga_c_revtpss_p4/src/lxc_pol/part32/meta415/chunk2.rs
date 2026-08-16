//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1444/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1444(t3291: f64, t6258: f64, t1082: f64, t19380: f64, t6271: f64, t73: f64, t4976: f64, t11249: f64, t6305: f64) -> (f64, f64, f64, f64) {
    let t19438 = t3291 * t6258;
    let t19443 = t1082 * t19380;
    let t19446 = t6271 * t73;
    let t19447 = t19446 * t4976;
    let t19450 = t6305 * t11249;
    (t19438, t19443, t19447, t19450)
}
