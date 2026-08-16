//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2065/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2065(t94669: f64, t94671: f64, t25894: f64, t94668: f64, t25950: f64, t25953: f64, t26069: f64, t94407: f64, t1445: f64, t25912: f64, t689: f64, t7282: f64, t9646: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94672 = t94669 * t94671;
    let t94674 = t25894 * t94668;
    let t94675 = t94674 * t94671;
    let t94677 = t25950 * t25953;
    let t94682 = 0.91399340044406952588e-2_f64 * t26069 * t94407;
    let t94694 = t689 * t25912 * t1445;
    let t94696 = t9646 * t7282;
    (t94672, t94674, t94675, t94677, t94682, t94694, t94696)
}
