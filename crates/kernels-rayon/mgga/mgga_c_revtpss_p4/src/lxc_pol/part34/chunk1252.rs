//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1252/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1252(t19696: f64, t7121: f64, t20016: f64, t25500: f64, t19463: f64, t1972: f64, t19976: f64, t25580: f64, t19900: f64, t7111: f64, t1058: f64, t29779: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t107048 = t19696 * t7121;
    let t107064 = t25500 * t20016;
    let t107072 = t19463 * t1972;
    let t107086 = t25580 * t19976;
    let t107101 = t7111 * t19900;
    let t107107 = t29779 * t1058;
    (t107048, t107064, t107072, t107086, t107101, t107107)
}
