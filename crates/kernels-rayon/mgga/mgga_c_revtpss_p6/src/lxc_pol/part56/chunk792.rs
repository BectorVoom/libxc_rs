//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 792/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk792(t25352: f64, t689: f64, t7049: f64, t786: f64, t789: f64, t2471: f64, t7018: f64, t25331: f64, t7058: f64, t25309: f64, t7063: f64, t7060: f64) -> (f64, f64, f64, f64, f64) {
    let t25353 = t689 * t25352;
    let t25355 = t786 * t7049;
    let t25356 = t25355 * t789;
    let t25362 = 0.13009920719177044025e-1_f64 * t7018 * t2471;
    let t25364 = 0.96373646535613327357e-2_f64 * t7058 * t25331;
    let t25365 = t7063 * t25309;
    let t25366 = t25365 * t7060;
    (t25353, t25356, t25362, t25364, t25366)
}
