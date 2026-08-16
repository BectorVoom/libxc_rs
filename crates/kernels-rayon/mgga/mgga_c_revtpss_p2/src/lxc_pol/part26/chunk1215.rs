//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1215/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1215(t2103: f64, t47567: f64, t1364: f64, t26338: f64, t786: f64, t26261: f64, t40270: f64, t25950: f64, t26271: f64, t10073: f64, t25920: f64, t26260: f64) -> (f64, f64, f64, f64, f64) {
    let t96473 = 0.81814717454467823679e-4_f64 * t47567 * t2103;
    let t96486 = t786 * t26338 * t1364;
    let t96491 = 0.96373646535613327356e-3_f64 * t40270 * t26261;
    let t96500 = t25950 * t26271;
    let t96503 = t10073 * t25920 * t26260;
    (t96473, t96486, t96491, t96500, t96503)
}
