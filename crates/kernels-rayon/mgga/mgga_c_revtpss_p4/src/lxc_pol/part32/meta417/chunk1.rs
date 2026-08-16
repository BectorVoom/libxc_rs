//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1448/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1448(t19450: f64, t19491: f64, t1043: f64, t6258: f64, t1089: f64, t3153: f64, t6305: f64) -> (f64, f64, f64, f64) {
    let t19492 = t19450 * t19491;
    let t19497 = t6258 * t1043;
    let t19498 = t19497 * t1089;
    let t19501 = t6305 * t3153;
    (t19492, t19497, t19498, t19501)
}
