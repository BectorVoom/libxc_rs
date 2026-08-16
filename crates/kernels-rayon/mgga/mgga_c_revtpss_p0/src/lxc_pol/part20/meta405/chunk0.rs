//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1498/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1498(t3089: f64, t42415: f64, t1087: f64, t11672: f64, t11711: f64, t1024: f64, t12003: f64, t10356: f64, t999: f64, t11744: f64, t3188: f64, t3181: f64, t675: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42416 = t42415 * t3089;
    let t42417 = t1087 * t42416;
    let t42421 = t11672 * t11711;
    let t42425 = t1024 * t12003;
    let t42428 = t10356 * t999;
    let t42439 = t3188 * t11744;
    let t42447 = t675 * t3181;
    (t42416, t42417, t42421, t42425, t42428, t42439, t42447)
}
