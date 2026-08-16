//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 796/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk796(t233: f64, t867: f64, t1949: f64, t7056: f64, t10073: f64, t1957: f64, t822: f64, t25386: f64, t676: f64, t837: f64, t25377: f64, t2718: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25402 = t867 * t233;
    let t25403 = t25402 * t1949;
    let t25404 = t7056 * t25403;
    let t25406 = 0.24093411633903331839e-3_f64 * t10073 * t25404;
    let t25410 = t1957 * t822;
    let t25411 = t25386 * t25410;
    let t25412 = t676 * t837;
    let t25413 = t25377 * t25412;
    let t25414 = t25411 * t25413;
    let t25416 = t867 * t2718;
    (t25406, t25410, t25411, t25412, t25413, t25414, t25416)
}
