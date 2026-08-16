//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1652/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1652(t1927: f64, t644: f64, t4144: f64, t9593: f64, t196: f64, t197: f64, t3821: f64, t2394: f64, t30: f64, t2411: f64) -> (f64, f64, f64, f64, f64) {
    let t25163 = t1927 * t644;
    let t25177 = t9593 * t4144;
    let t25188 = t3821 * t196 * t197;
    let t25198 = t30 * t2394;
    let t25207 = t2411 * t30;
    (t25163, t25177, t25188, t25198, t25207)
}
