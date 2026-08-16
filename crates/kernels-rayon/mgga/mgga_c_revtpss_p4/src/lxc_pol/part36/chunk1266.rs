//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1266/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1266(t1032: f64, t6041: f64, t867: f64, t786: f64, t7060: f64, t105936: f64, t92843: f64, t29658: f64, t686: f64, t72: f64, t7058: f64, t7064: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t105944 = t6041 * t1032;
    let t105945 = t105944 * t867;
    let t105946 = t786 * t105945;
    let t105947 = t105946 * t7060;
    let t105949 = t92843 * t105936;
    let t105953 = t29658 * t72 * t686;
    let t105954 = t7058 * t105953;
    let t105956 = t7064 * t105953;
    (t105944, t105945, t105947, t105949, t105954, t105956)
}
