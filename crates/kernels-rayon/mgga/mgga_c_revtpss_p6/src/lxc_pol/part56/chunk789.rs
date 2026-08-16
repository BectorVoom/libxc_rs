//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 789/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk789(t25296: f64, t7058: f64, t2453: f64, t7057: f64, t136: f64, t1958: f64, t2457: f64, t1954: f64, t9645: f64) -> (f64, f64, f64, f64) {
    let t25297 = t7058 * t25296;
    let t25299 = t2453 * t7057;
    let t25300 = t1958 * t136;
    let t25301 = t25300 * t2457;
    let t25303 = 0.17135234354032049604e-2_f64 * t25299 * t25301;
    let t25304 = t1954 * t9645;
    (t25297, t25301, t25303, t25304)
}
