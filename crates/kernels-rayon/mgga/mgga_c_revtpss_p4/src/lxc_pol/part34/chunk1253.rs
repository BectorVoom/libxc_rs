//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1253/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1253(t20020: f64, t7117: f64, t19907: f64, t7111: f64, t19912: f64, t27479: f64, t4845: f64, t1035: f64, t29807: f64, t29834: f64, t7166: f64, t1976: f64, t6305: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t107140 = t7117 * t20020;
    let t107154 = t7111 * t19907;
    let t107169 = t7111 * t19912;
    let t107188 = t27479 * t4845;
    let t107207 = t1035 * t29807;
    let t107212 = t29834 * t7166;
    let t107225 = t1976 * t6305;
    (t107140, t107154, t107169, t107188, t107207, t107212, t107225)
}
