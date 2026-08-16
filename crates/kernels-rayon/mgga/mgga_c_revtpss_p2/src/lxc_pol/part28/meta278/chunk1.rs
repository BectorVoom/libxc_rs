//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1245/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1245(t532: f64, t7933: f64, t1450: f64, t2014: f64, t2034: f64, t5542: f64, t118: f64, t1502: f64, t1519: f64, t1843: f64, t1911: f64, t1932: f64, t2007: f64, t2011: f64, t508: f64, t569: f64, t651: f64, t6985: f64, t7725: f64, t7731: f64, t7734: f64, t7737: f64, t7744: f64, t7746: f64, t7883: f64, t7894: f64, t7899: f64, t7903: f64) -> (f64, f64, f64, f64) {
    let t7934 = t532 * t7933;
    let t7935 = t7934 * t1450;
    let t7936 = t2014 * t7935;
    let t7937 = t2034 * t5542;
    let t7938 = t2014 * t7937;
    let t7939 = -t118 * t7883 - t1502 * t2007 - 2.0_f64 * t1519 * t6985 - t1843 * t1932 + t1911 * t2011 - t508 * t7725 + t569 * t7894 - 2.0_f64 * t651 * t7746 - t7731 - t7734 - t7737 - t7744 + t7899 + t7903 + t7936 - t7938;
    (t7934, t7935, t7937, t7939)
}
